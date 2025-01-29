# Bareops plugin api

Bareops extension are WASM files that can be dynamically added as plugins.

## Requirements

* cargo install cargo-componen
* rustup target add wasm32-wasi

## Plugin structure

See bareops/examples folder.

## Build

* cargo component build

## Credits

Thanks to https://github.com/benwis/wasip2_plugins/blob/main/plugin_host/src/main.rs
