FROM rust:latest

WORKDIR /app
RUN cargo run build --release