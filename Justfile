set shell := ["bash", "-uc"]

default: setup

# Setup local development environment
setup:
  #!/bin/bash
  if [[ "$(cargo 2>&1)" == *"rustup could not choose a version of cargo to run"* ]]; then
    rustup default 1.78.0
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
  cargo build --release --target aarch64-apple-darwin --package tbdex_uniffi
  cargo run --release --package tbdex_uniffi \
    --bin uniffi-bindgen \
    generate --library target/aarch64-apple-darwin/release/libtbdex_uniffi.dylib \
    --language kotlin \
    --out-dir target/bindgen-kotlin
  cp target/aarch64-apple-darwin/release/libtbdex_uniffi.dylib bound/kt/src/main/resources
  cp target/bindgen-kotlin/tbdex/sdk/rust/tbdex.kt bound/kt/src/main/kotlin/tbdex/sdk/rust