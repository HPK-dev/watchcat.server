FROM rust:1.76.0
WORKDIR /app/watchcat-server

COPY . .

ENV RUST_LOG debug

ENTRYPOINT ["cargo", "run"]


# FROM lukemathwalker/cargo-chef:latest-rust-1.76.0 AS chef
# WORKDIR /app
#
# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json
#
# FROM chef AS builder 
# COPY --from=planner /app/recipe.json recipe.json
# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json
# # Build application
# COPY . .
# RUN cargo build --release
#
# # We do not need the Rust toolchain to run the binary!
# FROM debian:bookworm-slim AS runtime
# WORKDIR /app
# COPY --from=builder /app/target/release/watchcat-server /usr/local/bin
#
# ENV RUST_LOG info
#
# ENTRYPOINT ["/usr/local/bin/watchcat-server"]
