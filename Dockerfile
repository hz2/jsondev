FROM rust:1.87-slim AS builder
WORKDIR /app

# cache deps layer
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/jsondev ./jsondev
# content, templates, and static are mounted as volumes via docker-compose
EXPOSE 3000
CMD ["./jsondev"]
