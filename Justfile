set shell := ["bash", "-uc"]

default: setup

# Setup local development environment
setup:
  #!/bin/bash
  git submodule update --init
  if [[ "$(cargo 2>&1)" == *"rustup could not choose a version of cargo to run"* ]]; then
    rustup default 1.78.0
    rustup target add aarch64-apple-darwin
  fi

# Build a release variant
build: setup
  cargo build --release

# Run all tests
test: setup
  cargo test --workspace

# Run linting, look for warnings and/or diffs in the output to correct
lint: setup
  cargo clippy --workspace
  cargo fmt

bind: setup
  just bind-kotlin

bind-kotlin: setup
  cargo build --release --package tbdex_uniffi --target aarch64-apple-darwin
  cp target/aarch64-apple-darwin/release/libtbdex_uniffi.dylib \
    bound/kt/src/main/resources/libtbdex_uniffi_aarch64_apple_darwin.dylib
  cargo run --release --package tbdex_uniffi \
    --bin uniffi-bindgen \
    generate --library bound/kt/src/main/resources/libtbdex_uniffi_aarch64_apple_darwin.dylib \
    --language kotlin \
    --out-dir target/bindgen-kotlin
  cp target/bindgen-kotlin/tbdex/sdk/rust/tbdex.kt bound/kt/src/main/kotlin/tbdex/sdk/rust/UniFFI.kt