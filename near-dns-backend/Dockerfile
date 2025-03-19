FROM rust:1.85.1-bullseye AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config \
  libssl-dev \
  libudev-dev

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
COPY --from=builder /usr/src/app/target/release/near-dns-backend /usr/local/bin/near-dns-backend

# Set the entry point to the application
ENTRYPOINT ["/usr/local/bin/near-dns-backend"]
