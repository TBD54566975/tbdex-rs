FROM --platform=linux/amd64 ubuntu:22.04

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    libssl-dev \
    pkg-config

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the source code to the container
WORKDIR /usr/src/myapp
COPY Cargo.toml ./
COPY bindings/tbdex_uniffi ./bindings/tbdex_uniffi
COPY crates/tbdex ./crates/tbdex

# Execute the build
RUN cargo build --release --package tbdex_uniffi

# Set the entrypoint, so that we can `docker cp` the build output
CMD tail -f /dev/null