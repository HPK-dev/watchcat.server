FROM rust:1.76.0
WORKDIR /app/watchcat-server

COPY src/ src/
COPY static/ static/
COPY Cargo.toml .

ENV RUST_LOG info

ENTRYPOINT ["cargo", "run", "--release"]
