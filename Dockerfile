FROM rust:1.78 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/task-manager-backend /usr/local/bin/task-manager-backend
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations
CMD ["task-manager-backend"]
