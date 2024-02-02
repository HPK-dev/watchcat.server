FROM rust:1.74.0
WORKDIR /app/watchcat-server

COPY src/ src/
COPY static/ static/
COPY Cargo.toml .

ENV RUST_LOG info

RUN cargo build --release

ENTRYPOINT ["cargo", "run"]
