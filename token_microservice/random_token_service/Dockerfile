# Use the official Rust image version 1.80.1 as a build stage
FROM rust:1.80.1 as builder

# Set the working directory inside the container
WORKDIR /usr/src/random_token_service

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies only
RUN cargo build --release && rm -f target/release/deps/random_token_service*

# Copy the actual source code
COPY src ./src

# Build the final executable
RUN cargo build --release

# Use the Debian Bookworm slim image as the runtime stage
FROM debian:bookworm-slim

# Install necessary shared libraries (if required)
RUN apt-get update && apt-get install -y build-essential libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/random_token_service/target/release/random_token_service /usr/local/bin/random_token_service

# Set the entrypoint for the container
ENTRYPOINT ["random_token_service"]
