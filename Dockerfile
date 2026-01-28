# Use a Rust base image for the builder stage
FROM rust:1.84-slim-bookworm as builder

WORKDIR /app

# Install system dependencies (needed for some crates)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Create a blank project to cache dependencies
RUN cargo init
COPY Cargo.toml Cargo.lock ./

# Copy migrations (needed for sqlx::migrate! macro at compile time)
COPY migrations ./migrations

# Build only dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the actual source code
COPY src ./src
# Build the application
# We need to touch the main.rs to ensure a rebuild
RUN touch src/main.rs
RUN cargo build --release

# Use a minimal base image for the runtime
# cc-debian12 includes the C standard library which Rust needs, but is very small
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/rust-teraform-backend /app/server

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["/app/server"]
