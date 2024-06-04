FROM mcr.microsoft.com/devcontainers/rust

RUN sudo apt-get update && apt-get upgrade -y
#RUN sudo apt-get install --no-install-recommends -y \

RUN rustup toolchain install nightly
RUN rustup default nightly

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

RUN rustup component add rust-analyzer

EXPOSE 8080

WORKDIR /workspace
