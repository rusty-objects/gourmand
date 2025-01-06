# gourmand recipe recommender

## Github setup

### author identity

* [setting up your email on github](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-email-preferences/setting-your-commit-email-address):
```
git config --global --edit
git commit --amend --reset-author # if already committed something
```

### Setting up ssh access
docs:
* options
    * [single repo Deploy Keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/managing-deploy-keys#set-up-deploy-keys)
    * [account wide SSH Keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)

```
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
```

If you generated a github specific path for the key, add the following to `.ssh/config`:
```
Host github.com
Hostname github.com
IdentityFile ~/.ssh/id_ed25519_github
IdentitiesOnly yes
AddKeysToAgent yes
```

## Project setup 
* [cloning a git repo](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)
```
git clone git@github.com:rusty-objects/gourmand.git
```

## VS Code setup, Rust
* [VS Code setup](https://users.rust-lang.org/t/setting-up-rust-with-vs-code/76907)
* [rust-analyzer docs](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
* [Format On Save](https://stackoverflow.com/questions/67859926/how-to-run-cargo-fmt-on-save-in-vscode)
