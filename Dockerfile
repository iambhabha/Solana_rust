# Use the official Rust image as a builder
FROM rust:1.76-slim-bookworm as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build dependencies - this is the critical step for caching
# We build the project dependencies first
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL via ca-certificates (needed for HTTPS requests) and SQLite
RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/solana_rust_api .
COPY --from=builder /usr/src/app/.env .

# Expose the application port
EXPOSE 3000

# Run the binary
CMD ["./solana_rust_api"]
