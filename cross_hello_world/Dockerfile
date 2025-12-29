# Start from the official Rust image
FROM rust:latest

# Install necessary tools for cross-compilation
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    build-essential \
    pkg-config \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Add the ARM64 cross-compilation target for Rust
RUN rustup target add aarch64-unknown-linux-gnu

# Set the working directory inside the container
WORKDIR /project

# Copy the Rust project into the container
COPY . .

# Default command to build the Rust project
# This will build for the Raspberry Pi 5's ARM64 architecture
CMD ["cargo", "build", "--release", "--target=aarch64-unknown-linux-gnu"]
