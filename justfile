set windows-shell := ["powershell", "-Command"]

setup:
    cargo install cargo-component

build-examples:
    just examples/build
