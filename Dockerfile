# BUILDER
FROM rust:1.59.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# RUNTIME
FROM rust:1.59.0-slim AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/zero2prod zero2prod
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./zero2prod"]
