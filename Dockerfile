FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock Makefile.toml ./
COPY src ./src

RUN cargo install cargo-make

RUN rustup target add x86_64-pc-windows-gnu && \
    apt-get update && apt-get install -y gcc-mingw-w64 && \
    cargo install cargo-make

RUN cargo fetch

