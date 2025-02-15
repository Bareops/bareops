# Bareops

## Build

### Requirements

* [Rust and Cargo](https://www.rust-lang.org/): currently we try to be compatible with latest but greatest Rust version
* [just](https://github.com/casey/just): we use `just` to ease compile tasks

### Compile

Just run `cargo build --release` to build the core CLI-application.

### Plugins

For now, there are only small example plugins.

To build the example WASM plugins, you need to run `just setup` once to install `cargo-components`.
Afterward build the examples with `just build-examples`.

## Run

To run the CLI application, you need to provide a file with the taskbook and 
folders where so search for the plugins.

Examples (called from root of repository after building a release build and the example plugins):

With cargo:

```bash
cargo run --release -- run -f examples/hello-world.bl --path target/wasm32-wasip1/ 
```

Or just the binary:

```bash
target/release/bareops -f examples/hello-world.bl --path target/wasm32-wasip1/ 
```

Or maybe if you are on Windows:

```bash
target/release/bareops.exe -f examples/hello-world.bl --path target/wasm32-wasip1/ 
```
