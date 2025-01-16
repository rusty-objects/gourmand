//! Recipe recommender
use std::fs;
use std::path::Path;

use aws_sdk_bedrockruntime::types::{
    ContentBlock, ConversationRole, ConverseOutput, Message, StopReason, SystemContentBlock,
    ToolConfiguration, ToolResultBlock, ToolResultContentBlock, ToolUseBlock,
};
use aws_sdk_bedrockruntime::Client;
use clap::Parser;
use log::{debug, warn};
use recipes::system_prompts::SYS_PROMPT2 as SYS_PROMPT;
use rusty_bedrock_lib::converse::tool_use::{self, ToolArgType};
use rusty_bedrock_lib::file;
use rusty_bedrock_lib::nova::canvas;
use shellfish::rustyline::DefaultEditor as DefaultEditorRusty;
use shellfish::{clap_command, handler::DefaultAsyncHandler, Shell};

/// Get recipe recommendations interactively.
///
/// Callers need permission for `bedrock:InvokeModel`
///
/// Example:
///     converse -p bedrock -o ~/Desktop -m us.amazon.nova-lite-v1:0
// these are the args for launching the shell
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, verbatim_doc_comment)]
struct CliArgs {
    /// AWS profile override
    ///
    /// AWS region and credentials are selected in the following sequence:
    ///
    /// 1/ Explicit Override:
    /// When this --profile option is specified, the named profile will be read from
    ///     ~/.aws/config and ~/.aws/credentials.
    ///
    /// 2/ Environment Variables, as described here:
    ///     https://docs.aws.amazon.com/cli/v1/userguide/cli-configure-envvars.html
    ///
    /// 3/ Default profile:
    /// Uses the default profile from ~/.aws/config and ~/.aws/credentials.
    ///
    /// See the AWS docs for more information:
    ///   https://docs.aws.amazon.com/sdkref/latest/guide/file-format.html
    ///   https://docs.aws.amazon.com/sdk-for-rust/latest/dg/region.html
    ///   https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credproviders.html
    #[clap(long)]
    aws_profile: Option<String>,

    /// Enable verbose mode (prints messages to bedrock)
    #[clap(short, long)]
    verbose: bool,

    /// Model or inference profile id to use
    ///
    /// Not all models support Converse.  Some models such as those in the Amazon
    /// Nova family are accessible in some Regions only through cross-region inference.
    /// For those, specify an inference profile id.  For example:
    ///
    /// Amazon Nova Lite:
    ///   model-id: amazon.nova-lite-v1:0
    ///   inference-profile-id: us.amazon.nova-lite-v1:0
    ///
    /// Anthropic Claude Sonnet v2
    ///   model-id: anthropic.claude-3-5-sonnet-20241022-v2:0
    ///   inference-profile-id: us.anthropic.claude-3-5-sonnet-20241022-v2:0
    ///
    /// See:
    ///   https://docs.aws.amazon.com/bedrock/latest/userguide/models-supported.html
    ///   https://docs.aws.amazon.com/bedrock/latest/userguide/conversation-inference-supported-models-features.html
    ///   https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html
    #[clap(
        short,
        long,
        default_value = "us.anthropic.claude-3-5-sonnet-20241022-v2:0",
        verbatim_doc_comment
    )]
    model: String,

    /// Output directory for any artifacts
    #[clap(short, long, default_value = ".")]
    output: String,

    /// List models enabled for your account
    ///
    /// https://docs.aws.amazon.com/bedrock/latest/APIReference/API_ListFoundationModels.html
    #[clap(short, long)]
    list: bool,
}

/// Send a message to the model
// sub command within the shell once it's launched
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct SayArgs {
    /// The prompt for your next turn in the conversation
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: CliArgs = CliArgs::parse();

    let verbosity = if cli.verbose { 3 } else { 2 };
    stderrlog::new()
        .verbosity(verbosity)
        .module("rusty_bedrock_lib")
        .module(module_path!())
        .init()
        .unwrap();

    // https://docs.rs/aws-sdk-bedrockruntime/latest/aws_sdk_bedrockruntime/
    let client = rusty_bedrock_lib::new_runtime_client(cli.aws_profile).await;

    // System prompt sets the tone for the conversation
    let system_prompt = Some(vec![SystemContentBlock::Text(SYS_PROMPT.to_string())]);

    let tools = mk_recipe_tramission_tool();
    debug!("tools:\n{:?}", tools);

    let mut state = ConversationState {
        model: cli.model.clone(),
        output: cli.output,
        client,
        verbose: cli.verbose,
        system_prompt,
        tools: Some(tools),
        messages: vec![],
    };

    // start with the model introducing itself
    handle_prompt(&mut state, "
    To begin, please introduce yourself and ask the user some basic questions about their preferences
    ".to_string()).await.unwrap();

    println!();

    // Define a shell
    let mut shell = Shell::new_with_async_handler(
        state,
        format!("[{}]\n> ", cli.model),
        DefaultAsyncHandler::default(),
        DefaultEditorRusty::new()?,
    );
    shell.commands.insert(
        "say",
        clap_command!(ConversationState, SayArgs, async |state, args: SayArgs| {
            handle_prompt(state, args.prompt)
        }),
    );
    shell.run_async().await?;

    Ok(())
}

// ==========================================
// Conversation Event Loop
// ==========================================

#[derive(Debug)]
pub struct ConversationState {
    pub model: String,
    pub output: String,
    pub client: Client, // bedrock client
    pub verbose: bool,
    pub system_prompt: Option<Vec<SystemContentBlock>>,
    pub messages: Vec<Message>,
    pub tools: Option<ToolConfiguration>,
}

async fn handle_prompt(
    state: &mut ConversationState,
    prompt: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut stop_reason, mut msg) =
        conversation_turn(state, ConversationTurnInput::Prompt(prompt)).await;

    // -------------------
    // Loop for tool output.  When we're done with tool requests we'll return,
    // which will cause the shell to wait for the next prompt from user input.
    // -------------------
    loop {
        // set content (mut) above
        // TODO move conversation turn here.  then process response (either
        // print or fetch a tool and prepare tool response content)
        // after that, check stop_reason.  if end_turn, return.  If tool_use,
        // loop again (with the content coming to the top of the loop for
        // conversation turn).  If it's anything else, then throw an error
        // or panic.
        //
        // TODO if anthropic slows you down, don't panic, just print

        debug!(">>> STOP REASON {} <<<", stop_reason);
        /*
        match stop_reason {
            StopReason::ContentFiltered => todo!(),
            StopReason::EndTurn => todo!(),
            StopReason::GuardrailIntervened => todo!(),
            StopReason::MaxTokens => todo!(),
            StopReason::StopSequence => todo!(),
            StopReason::ToolUse => todo!(),
            StopReason::Unknown(unknown_variant_value) => todo!(),
            _ => todo!(),
        }
        */
        let mut found_tool = false;
        let cloned = msg.clone();

        // TODO you forgot to look at stop reason
        // TODO also this looping construct is ugly as hell

        for content in cloned.content() {
            match content {
                ContentBlock::Document(_document_block) => todo!(),
                ContentBlock::GuardContent(_guardrail_converse_content_block) => {
                    warn!("unexpected guardrail")
                }
                ContentBlock::Image(_image_block) => warn!("<<<< unexpected image "),
                ContentBlock::Text(s) => println!("{}", s),
                ContentBlock::ToolResult(_tool_result_block) => {
                    warn!("unexpected tool result")
                }
                ContentBlock::ToolUse(tool_use_block) => {
                    // println!("PROCESSING TOOL USE");
                    let tool_use_response = handle_tool_use(state, tool_use_block).await;
                    (stop_reason, msg) = conversation_turn(
                        state,
                        ConversationTurnInput::ToolResponse(tool_use_response),
                    )
                    .await;
                    found_tool = true;
                }
                ContentBlock::Video(_video_block) => warn!("unexpected video"),
                _ => panic!("Unknown response ContentBlock: {:?}", content),
            }
        }
        if !found_tool {
            return Ok(());
        }
    }
}

#[derive(Debug)]
pub enum ConversationTurnInput {
    Prompt(String),
    ToolResponse(ToolResultBlock),
}
impl ConversationTurnInput {
    pub fn to_content(&self) -> ContentBlock {
        match self {
            ConversationTurnInput::Prompt(txt) => ContentBlock::Text(txt.to_string()),
            ConversationTurnInput::ToolResponse(tool) => ContentBlock::ToolResult(tool.clone()),
        }
    }
}

/// Adds the message (and the response message) to the conversation state
pub async fn conversation_turn(
    state: &mut ConversationState,
    turn: ConversationTurnInput,
) -> (StopReason, Message) {
    debug!("model: {}", state.model);
    debug!("{:?}", turn);

    // ===========================
    // Create a new message from the ConversationTurnInput
    // ===========================
    let msg = Message::builder()
        .role(ConversationRole::User)
        .content(turn.to_content())
        .build()
        .unwrap();

    state.messages.push(msg);

    // ===========================
    // Send request to bedrock with entire conversation history
    // ===========================
    let conversation = state
        .client
        .converse()
        .model_id(state.model.clone())
        .set_system(state.system_prompt.clone())
        .set_messages(Some(state.messages.clone()))
        .set_tool_config(state.tools.clone())
        .send()
        .await
        .unwrap();
    /*
    TODO: Don't crash on Throttling Exception
    thread 'main' panicked at src/cli/recipes_main.rs:227:10:
    called `Result::unwrap()` on an `Err` value: ServiceError(ServiceError { source: ThrottlingException(ThrottlingException
    { message: Some("Too many requests, please wait before trying again."), meta: ErrorMetadata { code: Some("ThrottlingException"),
     message: Some("Too many requests, please wait before trying again."), extras: Some({"aws_request_id":
     "a08a73eb-05c5-416f-b6f4-51f9cab3f35f"}) } }), raw: Response { status: StatusCode(429), headers: Headers { headers:
     {"date": HeaderValue { _private: H0("Thu, 16 Jan 2025 05:58:20 GMT") }, "content-type": HeaderValue {
     _private: H0("application/json") }, "content-length": HeaderValue { _private: H0("65") }, "x-amzn-requestid": HeaderValue
     { _private: H0("a08a73eb-05c5-416f-b6f4-51f9cab3f35f") }, "x-amzn-errortype": HeaderValue
      { _private: H0("ThrottlingException:http://internal.amazon.com/coral/com.amazon.bedrock/") }} }, body: SdkBody {
       inner: Once(Some(b"{\"message\":\"Too many requests, please wait before trying again.\"}")), retryable: true }, extensions:
       Extensions { extensions_02x: Extensions, extensions_1x: Extensions } } })
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */

    debug!("{:?}", conversation);

    // ===========================
    // Extract assistant's response onto the message history state, return it
    // ===========================
    let stop_reason = conversation.stop_reason().clone();
    if let Some(ConverseOutput::Message(msg)) = conversation.output() {
        assert_eq!(&ConversationRole::Assistant, msg.role());
        state.messages.push(msg.clone());
        debug!("{:?}", msg);
        return (stop_reason, msg.clone());
    } else {
        panic!("No output??");
    };
}

// ==========================================
// Tool Use
// ==========================================

pub fn mk_recipe_tramission_tool() -> ToolConfiguration {
    let name = "transmit_recipe".to_string();
    let description = "
    this tool transmits a recipe (ingredients, instructions, and shopping list), a prompt for an
    image generation model to produce an appetizing photo of the recipe, as well as a file stem for
    saving the actual data.  It will return the actual location so that you can respond to the user.
    "
    .to_string();

    let inputs = vec![
        tool_use::ToolArg::new(
            "recipe_details",
            "The actual recipe, including ingredients, instructions, and shopping list",
            ToolArgType::String,
            true,
        ) ,
        tool_use::ToolArg::new(
            "image_prompt",
            "A prompt suitable for an image generation model to produce an appetizing photo of final dish",
            ToolArgType::String,
            true,
        ),
        tool_use::ToolArg::new(
            "file_stem",
            "a file stem for this recipe, all lowercase, with words separated by underscores, 
             with a 4 digit random numberic appended to the end.  such as: banana_bread_#### 
             but with numbers in place of #",
            ToolArgType::String,
            true,
        ),
    ];
    tool_use::mk_tool(name, description, inputs)
}

// https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rustv1/examples/bedrock-runtime/src/bin/tool-use.rs#L190
pub async fn handle_tool_use(
    state: &mut ConversationState,
    tool_use: &ToolUseBlock,
) -> ToolResultBlock {
    debug!("tool use id: {:?}", tool_use.tool_use_id());
    debug!("tool name: {:?}", tool_use.name());

    if tool_use.name() != "transmit_recipe" {
        panic!("model asked for unexpected tool: {}", tool_use.name());
    }

    let input = tool_use.input();
    let input_map = input.as_object().unwrap();

    let file_stem = input_map
        .get("file_stem")
        .map_or("default".to_string(), |doc| {
            doc.as_string().unwrap().to_string()
        });

    let image_prompt = input_map
        .get("image_prompt")
        .map_or("default".to_string(), |doc| {
            doc.as_string().unwrap().to_string()
        });

    let recipe_details = input_map
        .get("recipe_details")
        .map_or("default".to_string(), |doc| {
            doc.as_string().unwrap().to_string()
        });

    let outdir = transmit_recipe(state, file_stem, image_prompt, recipe_details).await;

    ToolResultBlock::builder()
        .tool_use_id(tool_use.tool_use_id())
        .content(ToolResultContentBlock::Text(
            format!("written output to {}", outdir).to_string(),
        ))
        .build()
        .unwrap()
}

async fn transmit_recipe(
    state: &ConversationState,
    file_stem: String,
    image_prompt: String,
    recipe_details: String,
) -> String {
    // !!!!!!!!!!!!!!
    // We must sanitize the path because some of the input came from the model
    // !!!!!!!!!!!!!!
    let file_stem = file::sanitize(file_stem);
    let outdir = format!("{}/{}", state.output, file_stem).to_string();
    let mut files = vec![];

    let (_trace_id, images) = canvas::text_to_image(&state.client, image_prompt, None).await;
    for (idx, image) in images.into_iter().enumerate() {
        let path = format!("{}-{}.png", outdir, idx);
        rusty_bedrock_lib::file::write_base64(path.as_str(), image);
        files.push(path);
    }
    let txt_path = format!("{}.txt", outdir).to_owned();
    let expanded = rusty_bedrock_lib::file::expand(&txt_path);
    let _ = fs::write(Path::new(expanded.as_str()), recipe_details);
    files.push(txt_path.clone());
    outdir
}
