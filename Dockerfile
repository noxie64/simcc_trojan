FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock Makefile.toml ./
COPY src ./src

RUN cargo fetch

