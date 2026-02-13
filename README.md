[![Rust](https://github.com/PotatoMaster101/git-tools/actions/workflows/rust.yml/badge.svg)](https://github.com/PotatoMaster101/git-tools/actions/workflows/rust.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

# Git Tools
Collection of CLI tools for extending [Git](https://git-scm.com/).

## Tools
- [`git-todo`](./bins/git-todo/): manage todo lists for different branches
- [`git-user`](./bins/git-user/): manage user profiles for different repos

## Installing
```
cargo install --path bins/git-todo
cargo install --path bins/git-user
```

## Testing
```
cargo test --release
```
