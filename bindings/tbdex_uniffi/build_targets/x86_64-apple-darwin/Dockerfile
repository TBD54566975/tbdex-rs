# # Use the official Rust image as a base
# FROM rust:latest

# # Install necessary dependencies
# RUN apt-get update && \
#     apt-get install -y \
#     build-essential \
#     pkg-config \
#     libssl-dev \
#     cmake \
#     curl \
#     && rm -rf /var/lib/apt/lists/*

# # Install Homebrew
# RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# # Add Homebrew to the PATH
# ENV PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:${PATH}"

# # Install OpenSSL using Homebrew
# RUN brew install openssl@1.1

# # Set environment variables for OpenSSL
# ENV OPENSSL_DIR=/home/linuxbrew/.linuxbrew/opt/openssl@1.1
# ENV PKG_CONFIG_ALLOW_CROSS=1
# ENV PKG_CONFIG_PATH=$OPENSSL_DIR/lib/pkgconfig

# # Install the x86_64-apple-darwin target and toolchain
# RUN rustup target add x86_64-apple-darwin
# RUN rustup toolchain install stable-x86_64-apple-darwin

# # Set working directory
# WORKDIR /app

# # Copy project files
# COPY Cargo.toml Cargo.lock ./
# COPY crates/tbdex crates/tbdex
# COPY crates/pfi_exemplar_integration_test crates/pfi_exemplar_integration_test
# COPY bindings/tbdex_uniffi bindings/tbdex_uniffi

# WORKDIR /app/bindings/tbdex_uniffi

# # Build the Rust crate
# RUN cargo build --release --target x86_64-apple-darwin

# CMD tail -f /dev/null




# FROM --platform=linux/amd64 alpine:latest

# # Install build dependencies
# RUN apk add --no-cache \
#     build-base \
#     cmake \
#     pkgconfig \
#     openssl-dev \
#     curl \
#     wget \
#     git \
#     ca-certificates \
#     linux-headers

# # Install rustup and set default toolchain
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"

# RUN rustup default 1.78.0
# RUN rustup target add x86_64-apple-darwin

# # Set working directory
# WORKDIR /app

# # Copy project files
# COPY Cargo.toml Cargo.lock ./
# COPY crates/tbdex crates/tbdex
# COPY crates/pfi_exemplar_integration_test crates/pfi_exemplar_integration_test
# COPY bindings/tbdex_uniffi bindings/tbdex_uniffi

# WORKDIR /app/bindings/tbdex_uniffi

# # Set environment variables to help linker find OpenSSL libraries
# ENV OPENSSL_DIR=/usr
# ENV OPENSSL_LIB_DIR=/usr/lib
# ENV OPENSSL_INCLUDE_DIR=/usr/include
# ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig

# # Build the project
# RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --target x86_64-apple-darwin

# CMD tail -f /dev/null




# FROM multiarch/crossbuild

# # Install Rust
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"

# # Install the x86_64-apple-darwin target
# RUN rustup target add x86_64-apple-darwin

# # Set environment variables for cross-compilation
# ENV TARGET_CC=x86_64-apple-darwin12-clang
# ENV TARGET_CXX=x86_64-apple-darwin12-clang++
# ENV TARGET_AR=x86_64-apple-darwin12-ar
# ENV TARGET_LINKER=x86_64-apple-darwin12-clang

# # Set working directory
# WORKDIR /app

# # Copy project files
# COPY Cargo.toml Cargo.lock ./
# COPY crates/tbdex crates/tbdex
# COPY crates/pfi_exemplar_integration_test crates/pfi_exemplar_integration_test
# COPY bindings/tbdex_uniffi bindings/tbdex_uniffi

# WORKDIR /app/bindings/tbdex_uniffi

# # Build the project
# RUN cargo build --release --target x86_64-apple-darwin

# CMD tail -f /dev/null


# Use a base image with necessary build tools
FROM ubuntu:20.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    clang \
    cmake \
    git \
    wget \
    curl \
    xz-utils \
    libssl-dev \
    pkg-config \
    libz-dev \
    python3

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install osxcross
RUN git clone https://github.com/tpoechtrager/osxcross.git /opt/osxcross
WORKDIR /opt/osxcross
RUN UNATTENDED=yes ./tools/get_dependencies.sh
RUN wget https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX10.11.sdk.tar.xz
RUN mv MacOSX10.11.sdk.tar.xz tarballs/
RUN UNATTENDED=yes OSX_VERSION_MIN=10.11 ./build.sh

# Set up environment variables for osxcross
ENV PATH="/opt/osxcross/target/bin:${PATH}"
ENV OSXCROSS_MP_INC=1

# # Create a directory for the project
# WORKDIR /project

# # Copy the project files
# COPY . /project

# Set working directory
WORKDIR /app

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY crates/tbdex crates/tbdex
COPY crates/pfi_exemplar_integration_test crates/pfi_exemplar_integration_test
COPY bindings/tbdex_uniffi bindings/tbdex_uniffi

WORKDIR /app/bindings/tbdex_uniffi

# Build the Rust project for x86_64-apple-darwin
RUN cargo build --target x86_64-apple-darwin --release

# The compiled binaries will be in /project/target/x86_64-apple-darwin/release
