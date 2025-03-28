FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust_kafka /app
CMD ["/app"]