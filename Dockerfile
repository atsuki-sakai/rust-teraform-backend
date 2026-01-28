# Builder stage
FROM rust:1.93-slim-bookworm AS chef
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Copy application source code
COPY . .
# Build application
RUN cargo build --release --bin rust-teraform-backend

# Runtime stage
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/rust-teraform-backend /app/server
EXPOSE 8080
CMD ["/app/server"]
