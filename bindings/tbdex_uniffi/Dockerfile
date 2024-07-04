FROM rust:latest

RUN apt-get update && \
    apt-get install -y musl-tools llvm && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY .cargo/ Cargo.toml Cargo.lock ./
COPY crates/tbdex crates/tbdex
COPY crates/pfi_exemplar_integration_test crates/pfi_exemplar_integration_test
COPY bindings/tbdex_uniffi bindings/tbdex_uniffi

WORKDIR /app/bindings/tbdex_uniffi

RUN cargo build --release --target x86_64-unknown-linux-musl

CMD tail -f /dev/null
