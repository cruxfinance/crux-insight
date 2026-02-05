# Stage 1: Build the Rust project
FROM rust:slim-bullseye as builder

RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev build-essential

WORKDIR /app

# Copy only manifests and lock file first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY chain-indexer/Cargo.toml chain-indexer/Cargo.toml
COPY ergo-node-client/Cargo.toml ergo-node-client/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml

# Create dummy source files so cargo can resolve and build dependencies
RUN mkdir -p chain-indexer/src ergo-node-client/src migration/src && \
    echo "fn main() {}" > chain-indexer/src/main.rs && \
    echo "" > ergo-node-client/src/lib.rs && \
    echo "" > migration/src/lib.rs

# Build dependencies only - this layer is cached until Cargo.toml/Cargo.lock change
RUN cargo build --release --package chain-indexer

# Remove dummy build artifacts so the real source gets compiled
RUN rm -rf chain-indexer/src ergo-node-client/src migration/src \
    target/release/.fingerprint/chain-indexer-* \
    target/release/.fingerprint/ergo-node-client-* \
    target/release/.fingerprint/migration-* \
    target/release/deps/chain_indexer-* \
    target/release/deps/libergo_node_client-* \
    target/release/deps/libmigration-* \
    target/release/chain-indexer

# Copy the real source code
COPY . .

# Build the actual binaries (only workspace crates recompile)
RUN cargo build --release --package chain-indexer

# Stage 2: Create the final lightweight image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates openssl libssl-dev

WORKDIR /app

# Copy the compiled binaries from the builder stage
COPY --from=builder /app/target/release/chain-indexer /app/
COPY --from=builder /app/config /app/config
COPY --from=builder /app/local.toml /app/local.toml

CMD ["./chain-indexer"]
