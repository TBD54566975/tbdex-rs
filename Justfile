set shell := ["bash", "-uc"]

default: setup

# Setup local development environment
setup:
  #!/bin/bash
  if [[ "$(cargo 2>&1)" == *"rustup could not choose a version of cargo to run"* ]]; then
    rustup default stable
  fi

# Build a release variant
build: setup
  cargo build --release

# Run all tests
test: setup
  cargo test

# Run linting, look for warnings and/or diffs in the output to correct
lint: setup
  cargo clippy --workspace
  cargo fmt -- --check

# Run formatting
fmt: setup
  cargo fmt