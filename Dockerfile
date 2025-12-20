# Build stage
FROM rust:1.92.0-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY . .

# Build the application
RUN cargo build --release --bin api

# Final stage
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api /app/ausmalbilder_backend_rust

# Copy configuration
COPY config.yml /app/config.yml

# Expose the port
EXPOSE 8081

# Run the application
CMD ["/app/ausmalbilder_backend_rust"]
