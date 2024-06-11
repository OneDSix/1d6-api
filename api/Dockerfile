FROM ubuntu:latest
WORKDIR /app

# Install needed dependencies
# then install Rust and add it to the path
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env

# Copy the Rust application source code to the container
COPY . .

# Build the Rust application
RUN ~/.cargo/bin/cargo build --release

# Set the command to run your Rust application
CMD ["./target/release/onedsix-api"]
