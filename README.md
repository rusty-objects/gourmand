# gourmand recipe recommender

## Git
### [setting up author identity](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-email-preferences/setting-your-commit-email-address):
```
git config --global --edit
git commit --amend --reset-author # if already committed something
```
### Setting up ssh access
docs
* [managing single repo deploy keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/managing-deploy-keys#set-up-deploy-keys)
* [managing account-level ssh keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)
* [generating private key](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent#generating-a-new-ssh-key)
```
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
```

If you generated a github specific path for the key, add the following to `.ssh/config`:
```
Host github.com
Hostname github.com
IdentityFile ~/.ssh/id_ed25519_github
IdentitiesOnly yes # see NOTES below
AddKeysToAgent yes
```

### Cloning repo
* [cloning a git repo](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)
```
git clone https://github.com/rusty-objects/gourmand
```

### Rust in VS Code
* [VS Code setup](https://users.rust-lang.org/t/setting-up-rust-with-vs-code/76907)
* [Format On Save](https://stackoverflow.com/questions/67859926/how-to-run-cargo-fmt-on-save-in-vscode)
* [rust-analyzer docs](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
