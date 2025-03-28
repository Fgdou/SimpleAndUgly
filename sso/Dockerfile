FROM rust AS builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/sso /
CMD ["./sso"]