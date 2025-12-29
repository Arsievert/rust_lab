# Rust Cross-Compilation with Docker for Raspberry Pi 5

This project uses a Docker container based on Rust’s official image with ARM64 cross-compilation support to build Rust programs for a Raspberry Pi 5 running a 64-bit Yocto-based image.

## Prerequisites

Before you begin, ensure the following are installed on your machine:

1. **Docker**: Download and install Docker:
   - [Install Docker](https://docs.docker.com/get-docker/)
2. **Rust** (optional, for managing your project locally):
   - [Install Rust](https://rustup.rs)

## Project Setup

1. **Create Your Rust Project:**
   - If you don’t already have a Rust project, you can create one:
     ```bash
     cargo init --bin my-rust-project
     ```
   - Navigate into your project directory:
     ```bash
     cd my-rust-project
     ```

2. **Prepare the Dockerfile:**
   - Save the provided `Dockerfile` in the root of your Rust project, in the same directory as `Cargo.toml`.

---

## Build the Docker Image

To set up a Docker container for cross-compilation:

1. **Build the Docker Image:**
   Run the following command in the directory containing the `Dockerfile`:
   ```bash
   docker build -t rust-arm64-cross .
   ```

   This builds a Docker image with all necessary dependencies for cross-compiling Rust programs for ARM64-based platforms.

---

## Using the Container to Compile Rust Code

Once the Docker image is built:

1. **Run the Container:**
   Use the following command to compile your Rust project for the Raspberry Pi 5:
   ```bash
   docker run --rm -v $(pwd):/project rust-arm64-cross
   ```

   - `$(pwd)`: Mounts the current project directory into the container.
   - `/project`: Inside the container, your project files are accessible here.

2. **Find the Compiled Binary:**
   After running the build, you can find the compiled binary in the `target/aarch64-unknown-linux-gnu/release/` directory of your project folder:
   ```
   target/aarch64-unknown-linux-gnu/release/your-binary-name
   ```
---

## Customization

1. **Add Additional Build Dependencies:**
   If your project requires external libraries (e.g., OpenSSL), modify the `Dockerfile` by adding them:
   ```dockerfile
   RUN apt-get update && apt-get install -y libssl-dev
   ```

2. **Use Specific Rust Versions:**
   Modify the `FROM` line in the `Dockerfile` to specify a specific Rust version:
   ```dockerfile
   FROM rust:1.72
   ```