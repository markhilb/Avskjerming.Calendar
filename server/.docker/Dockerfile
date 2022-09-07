FROM rust:1.63-slim AS builder
ARG build_mode

WORKDIR /app

RUN apt update && apt dist-upgrade -y && apt install -y musl-tools libssl-dev pkg-config

# Copy only cargo & dummy lib/main files to cache crates
COPY Cargo* ./
COPY application/Cargo* application/

RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN mkdir application/src && touch application/src/lib.rs

RUN cargo build --release

COPY . .

# Update modified date to make cargo recompiles
RUN touch -am application/src/lib.rs src/main.rs

RUN cargo build --release


FROM scratch
COPY --from=builder /app/target/release/server .
CMD ["./server"]
