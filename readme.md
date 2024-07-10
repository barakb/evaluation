[![Build](https://github.com/barakb/evaluation/actions/workflows/build.yml/badge.svg)](https://github.com/barakb/evaluation/actions/workflows/build.yml)
[![Renovate enabled](https://img.shields.io/badge/renovate-enabled-brightgreen.svg)](https://renovatebot.com/)

### Install Rust on macOS

Use `rustup` to install and manage Rust on macOS. Run the following command in the
terminal: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Build the source

`cargo build`

### Build and run

`cargo run`

### Watch and run while developing

one time installation of the cargo-watch crate
`cargo binstall cargo-watch`

after that, run the following command to watch and run the code while developing

```bash
cargo watch -q -c -x run
```

### Asking for evaluation

```bash
cargo run -- ask -y 12 -p 16 -g 10
```
