# Build stage
FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/task-manager-backend /usr/local/bin/task-manager-backend
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations
ENV RUST_LOG=info
CMD ["task-manager-backend"]
