FROM rust:1.83-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin activedb-container

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/activedb-container /usr/local/bin/
EXPOSE 6969
CMD ["activedb-container"]
