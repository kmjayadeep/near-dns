FROM rust:1.84.1-bullseye AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config \
  libssl-dev \
  libudev-dev

# Build the dependencies
RUN cargo build --release

# Remove the dummy source file
RUN rm -rf src

# Copy the rest of the application source code
COPY . .

# Build the application
RUN cargo build --release

# Use a slim runtime image
FROM debian:trixie-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built application from the builder image
COPY --from=builder /usr/src/app/target/release/near-dns-backend-rust /usr/local/bin/near-dns-backend

# Set the entry point to the application
ENTRYPOINT ["/usr/local/bin/near-dns-backend"]
