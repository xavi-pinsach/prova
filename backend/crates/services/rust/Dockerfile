FROM rust:1.92 as builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY contracts/protobuf/v1 ./proto
COPY crates/services/rust/Cargo.toml crates/services/rust/Cargo.lock* ./
COPY crates/services/rust/build.rs ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

COPY crates/services/rust/src ./src

RUN touch src/main.rs && cargo build --release

FROM rust:1.92-slim

# Install minimal runtime libraries for mock binary
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/prova-rust /usr/local/bin/

EXPOSE 50051

CMD ["prova-rust"]
