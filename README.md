# Work in progress <!-- omit in toc -->

This repo is not ready for consumption, and is under heavy development

- [Getting Started](#getting-started)

## Getting Started

```shell
just setup # this will configure your rust environment
```

```shell
just --list
```

```shell
RUSTFLAGS="-C link-arg=-static-libgcc -C link-arg=-static-libstdc++ -C link-arg=-mmacosx-version-min=10.7" cargo build --target aarch64-apple-darwin --release --package tbdex_uniffi
```

```shell
rustup target add x86_64-apple-darwin
```

```shell
RUSTFLAGS="-C link-arg=-static-libgcc -C link-arg=-static-libstdc++ -C link-arg=-mmacosx-version-min=10.7" cargo build --target x86_64-apple-darwin --release --package tbdex_uniffi
```

# ubuntu

```shell
brew tap messense/macos-cross-toolchains
```

```shell
brew install x86_64-unknown-linux-gnu
```

```shell
rustup target add x86_64-unknown-linux-gnu
```

```shell
export CC_x86_64_unknown_linux_gnu=x86_64-linux-gnu-gcc
export CXX_x86_64_unknown_linux_gnu=x86_64-linux-gnu-g++
export AR_x86_64_unknown_linux_gnu=x86_64-linux-gnu-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

RUSTFLAGS="-C link-arg=-static-libgcc -C link-arg=-static-libstdc++" \
cargo build --target x86_64-unknown-linux-gnu --release --package tbdex_uniffi
```

# alpine

```shell
brew tap messense/macos-cross-toolchains
brew install x86_64-unknown-linux-musl
rustup target add x86_64-unknown-linux-musl
```

```shell
export CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc
export CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
export AR_x86_64_unknown_linux_musl=x86_64-linux-musl-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc

RUSTFLAGS="-C target-feature=-crt-static -C link-arg=-static-libgcc -C link-arg=-static-libstdc++" \
cargo build --target x86_64-unknown-linux-musl --release --package tbdex_uniffi
```