# --- Stage 1: Build ---
FROM rust:1.87-slim as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml ./
COPY crates ./crates

RUN cargo build -p cli --release

# --- Stage 2: Runtime ---
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/zyncdb .

CMD ["./zyncdb"]
