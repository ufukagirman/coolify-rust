FROM rust:1.83-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/deps/coolify_rust*

COPY src ./src
RUN cargo build --release

FROM alpine:3.20
RUN adduser -D -u 10001 app
WORKDIR /app
COPY --from=builder /app/target/release/coolify-rust /app/coolify-rust
USER app
EXPOSE 8080
CMD ["/app/coolify-rust"]
