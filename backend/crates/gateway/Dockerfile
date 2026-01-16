FROM rust:1.92 as builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV SQLX_OFFLINE=true

COPY contracts/protobuf/v1 ./proto
COPY gateway/Cargo.toml gateway/Cargo.lock* ./
COPY gateway/build.rs ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

COPY gateway/src ./src

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/prova-gateway /usr/local/bin/

EXPOSE 3000

CMD ["prova-gateway"]
