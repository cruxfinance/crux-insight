# Stage 1: Build the Rust project
FROM rust:latest as builder

# Create a new directory to copy the source code into
WORKDIR /app

# Copy the rest of the source code
COPY . .

# Build the individual binaries
RUN cargo build --release --package chain-indexer

# Stage 2: Create the final lightweight image
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binaries from the builder stage
COPY --from=builder /app/target/release/chain-indexer /app/
COPY --from=builder /app/config /app/config

# Specify the default command to run when the container starts
CMD ["./chain-indexer"]

# Optionally, you can expose ports if any of your binaries listen on specific ports
# EXPOSE 8080
