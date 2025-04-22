FROM rust:1.70 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/seu_programa /usr/local/bin/seu_programa

CMD ["devto_api"]
