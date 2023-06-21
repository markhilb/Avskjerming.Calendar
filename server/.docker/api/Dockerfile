FROM lukemathwalker/cargo-chef:latest-rust-1.70.0-alpine AS chef

WORKDIR /app

FROM chef AS planner
COPY src .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY src .
RUN cargo build --release --bin api

# RUN apt update -y && \
#     apt install -y \
#     git \
#     libssl1.1 \
#     libssl-dev \
#     protobuf-compiler \
#     pkg-config && \
#     git config --global credential.helper store

FROM gcr.io/distroless/cc

COPY src/api/config config

COPY --from=builder /app/target/release/api .

CMD ["./api"]
