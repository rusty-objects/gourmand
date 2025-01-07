# Gourmand recipe recommender

## Bedrock References
* [Bedrock API Reference](https://docs.aws.amazon.com/bedrock/latest/APIReference/welcome.html)
* [Bedrock Rust SDK](https://github.com/awslabs/aws-sdk-rust) ([crate](https://github.com/awslabs/aws-sdk-rust))

## Github setup

### author identity

[setting up your email on github](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-email-preferences/setting-your-commit-email-address)
```
git config --global --edit
git commit --amend --reset-author # if already committed something with the wrong email
```

### Setting up ssh access
options
* [single repo Deploy Keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/managing-deploy-keys#set-up-deploy-keys)
* [account wide SSH Keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)

```
# swap out your preferred email
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
```

If you generated a github specific path for the keypair, [add the following](https://stackoverflow.com/questions/4565700/how-to-specify-the-private-ssh-key-to-use-when-executing-shell-command-on-git) to `.ssh/config`:
```
Host github.com
IdentityFile ~/.ssh/id_ed25519_github # or whatever you called it
IdentitiesOnly yes
AddKeysToAgent yes
```

## Project setup 
[cloning a git repo](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)
```
git clone git@github.com:rusty-objects/gourmand.git
```

## VS Code setup
* Rust
    * [VS Code rust setup](https://users.rust-lang.org/t/setting-up-rust-with-vs-code/76907)
    * [rust-analyzer docs](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    * [Format On Save](https://stackoverflow.com/questions/67859926/how-to-run-cargo-fmt-on-save-in-vscode)
* Hold down key repeats input ([StackOverflow][https://stackoverflow.com/questions/39972335/how-do-i-press-and-hold-a-key-and-have-it-repeat-in-vscode])
```
# holding down repeats
defaults write com.microsoft.VSCode ApplePressAndHoldEnabled -bool false 

# holding down does NOT repeat
defaults write com.microsoft.VSCode ApplePressAndHoldEnabled -bool true
```