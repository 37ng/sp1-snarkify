# Use a smaller, specific version of the Rust image for building
FROM --platform=linux/amd64 rust:latest AS builder

# Create a new empty project (optional, if you don't need this step)
# RUN USER=root cargo new --bin snarkify
WORKDIR /workspace

# Copy the entire workspace
COPY . .

# Install SP1 Rust Toolchain
SHELL ["/bin/bash", "-c"]
RUN curl -L https://sp1.succinct.xyz | bash
RUN ls
RUN ["/root/.sp1/bin/sp1up"]


# Build only the snarkify binary
WORKDIR /workspace/script
RUN cargo build --release --bin snarkify

# Use a slim variant for the runtime stage to reduce size
FROM --platform=linux/amd64 ubuntu:22.04

# Copy the build artifact from the build stage
COPY --from=builder /workspace/target/release/snarkify /usr/local/bin/snarkify

# Set the startup command to run your binary
CMD ["/usr/local/bin/snarkify"]