FROM rust:1.68.0-slim-buster

RUN cargo install sqlx-cli cargo-make && \
  apt update && \
  apt install -y protobuf-compiler

USER root

ENTRYPOINT ["./docker-entrypoint.sh"]
