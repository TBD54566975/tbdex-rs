set shell := ["bash", "-uc"]

default: setup

# Setup local development environment
setup:
  rustup default stable

# Build a release variant
build:
  cargo build --release

# Run all tests
test:
  cargo test

# Run linting, look for warnings in the output to correct
lint:
  cargo clippy --workspace
  cargo fmt