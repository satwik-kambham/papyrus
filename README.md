# Papyrus

An elegant code editor inspired by atom with syntax highlighting,
integrated terminal and more.

## Build instructions

### Prerequisites

You will need the following tools installed:

- NodeJS
- Rust

Run `npm install` to install the necessary node dependencies

Install the tauri cli using:

```bash
cargo install tauri-cli
```

### Run in development mode

Run `cargo tauri dev` to start the application in development mode.

### Build production installers

Run `cargo tauri build` to build the executables for your current platform.

The installers can be found under `./src-tauri/target/release/bundle`.
