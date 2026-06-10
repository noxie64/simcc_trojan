FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock Makefile.toml installer.nsi ./
COPY src ./src

RUN cargo install cargo-make

RUN rustup target add x86_64-pc-windows-gnu && \
    apt-get update && apt-get install -y gcc-mingw-w64 nsis && \
    cargo install cargo-make

RUN cargo fetch

