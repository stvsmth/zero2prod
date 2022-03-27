FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y


## PLANNER 
FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json


## BUILDER
#  ... build our project dependencies, not our application!
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
#  ... if our dependency tree stays the same, all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true

# ... build our code in its own layer
RUN cargo build --release --bin zero2prod

## RUNTIME
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
