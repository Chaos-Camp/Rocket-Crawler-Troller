# Builder stage
FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

# Install musl-tools to get musl-gcc
RUN apt-get update && apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target x86_64-unknown-linux-musl

# Runner stage
FROM alpine:latest

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rocket-crawler-troll /app/rocket-crawler-troll

CMD ["/app/rocket-crawler-troll"]
