FROM ubuntu:24.04

RUN apt update
RUN apt install -yy curl nodejs git build-essential
RUN bash -c 'curl https://sh.rustup.rs -sSf | sh -s -- -y'
ENV PATH="$PATH:/root/.cargo/bin"


RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall cargo-component
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /bin
