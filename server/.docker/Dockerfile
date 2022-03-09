FROM rust:latest AS builder
ARG build_mode

WORKDIR /app

RUN apt update && apt dist-upgrade -y && apt install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

# Copy only cargo & dummy lib/main files to cache crates
COPY Cargo* ./
COPY application/Cargo* application/

RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN mkdir application/src && touch application/src/lib.rs

RUN cargo install --target x86_64-unknown-linux-musl --path .


COPY . .

# Update modified date to make cargo recompile
RUN touch -am application/src/lib.rs src/main.rs

RUN cargo install --target x86_64-unknown-linux-musl --path .


FROM scratch
COPY --from=builder /usr/local/cargo/bin/server .
CMD ["./server"]
