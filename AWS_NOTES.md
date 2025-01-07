# AWS

## Bedrock Features
* [Agents](https://aws.amazon.com/bedrock/agents/): GenAI application workflows.  [AI Agents v Agentic AI](https://medium.com/@elisowski/ai-agents-vs-agentic-ai-whats-the-difference-and-why-does-it-matter-03159ee8c2b4)
* [Guardrails](https://aws.amazon.com/bedrock/guardrails/): filter PII, prevent harmful topics, prevent hallucinations
* [Intelligent Prompt Routing](https://aws.amazon.com/bedrock/intelligent-prompt-routing/): within a model family, route a prompt to the cheapest model that won't degrade results
* [Knowledge Bases](https://aws.amazon.com/bedrock/knowledge-bases/): For RAG
* [Marketplace](https://aws.amazon.com/bedrock/marketplace/): Model marketplace
* [Model Distillation](https://aws.amazon.com/bedrock/model-distillation/): Fine tunes smaller "student models" by using a larger "teacher model" and a set of customer provided prompts.
* [Prompt Caching](https://aws.amazon.com/bedrock/prompt-caching/): When context prefixes are identical between invocations, will reuse results.
* [Prompt Management](https://aws.amazon.com/bedrock/prompt-management/): Server-side prompt management

## SageMaker AI Features
* [Studio](https://aws.amazon.com/sagemaker-ai/studio/): Web IDE, where you'll find jupyter, etc
* [Jump Start](https://aws.amazon.com/sagemaker-ai/jumpstart/): model playground
* [Data Zone](https://aws.amazon.com/datazone/): Data set sharing
* [Model Cards](https://docs.aws.amazon.com/sagemaker/latest/dg/model-cards.html): Standardized description for model details.  [huggingface guidebook](https://huggingface.co/docs/hub/en/model-card-guidebook)

## Other AWS Services
### AIML
* [Comprehend](https://aws.amazon.com/comprehend/): NLU
* [Forecast](https://aws.amazon.com/forecast/): Forecasting
* [Lex](https://aws.amazon.com/lex/): Chatbot builder
* [Personalize](https://aws.amazon.com/personalize/): Recommendations
* [Polly](https://aws.amazon.com/polly/): Speech generation
* [Q Developer](https://aws.amazon.com/q/developer/): Formerly Code Whisperer, AI coding assistant
    *  [vscode setup](https://marketplace.visualstudio.com/items?itemName=AmazonWebServices.amazon-q-vscode)
* [Rekognition](https://aws.amazon.com/rekognition/): image/video analysis
* [Textract](https://aws.amazon.com/textract/): OCR
* [Transcribe](https://aws.amazon.com/transcribe/): Speech recognition
* [Translate](https://aws.amazon.com/translate/): NL translation

### Search Services
* [Kendra](https://aws.amazon.com/kendra/): Optimized for semantic search across unstructured data
* [OpenSearch](https://aws.amazon.com/opensearch-service/): Fast search of structured data

## Bedrock featured Model Vendors (2024)

* AI21 ([website](https://www.ai21.com/) | [Bedrock](https://aws.amazon.com/bedrock/ai21/)) 
    * Jamba 1.5 Mini
    * Jamba 1.5 Large
    * Jamba Instruct: state space model
* Amazon ([website](https://aws.amazon.com/ai/generative-ai/nova/))
    * Nova Micro: text to text
    * Nova Lite: MM to text
    * Nova Pro: MM to text
    * Nova Premier: soon
    * Nova Canvas: text/image to image
    * Nova Reel: text/image to video
* Anthropic ([website](https://www.anthropic.com/) | [Bedrock](https://aws.amazon.com/bedrock/claude/))
    * text only
    * Claude 3.5 Sonnet: advanced, can code and interact with screens
    * Claude 3.5 Haiku: lighter weight than Sonnet, faster than Opus
    * Claude 3 Opus
* Cohere ([website](https://cohere.com/) | [Bedrock](https://aws.amazon.com/bedrock/cohere/))
    * Rerank 3.5: reranks vector search results
    * Command R+: bigger than R 
    * Command R
    * Embed: vector generation
* Meta ([website](https://www.llama.com/) | [Bedrock](https://aws.amazon.com/bedrock/llama/))
    * Llama 3.3 70B: text only
    * Llama 3.2 90B: visual reasoning
    * Llama 3.2 11B: smaller than 90B
    * Llama 3.2 3B, 1B: optimized for edge/local devices
* Mistral ([website](https://mistral.ai/) | [Bedrock](https://aws.amazon.com/bedrock/mistral/))
    * text only
    * Mistral Large 2
    * Mistral Large 
    * Mistral Small
    * Mistral 8x7B: MoE
    * Mistral 7B
* Stability AI ([website](https://stability.ai/) | [Bedrock](https://aws.amazon.com/bedrock/stability-ai/))
    * image generation
    * Stable Diffusion 3.5 Large
    * Stable Image Ultra
    * Stable Image Core
* [COMING SOON] poolside ([website](https://poolside.ai/) | [Bedrock](https://aws.amazon.com/bedrock/poolside/)) 
    * code generation
    * malibu: complex coding tasks, documentation, refactoring, etc
    * point: code completion
* [COMING SOON] Luma ([website](https://lumalabs.ai/) | [Bedrock](https://aws.amazon.com/bedrock/luma-ai/)) 
    * video generation
    * Luma Ray 2
