FROM rust:1.68.0-slim-buster

RUN cargo install sqlx-cli cargo-make

USER root

ENTRYPOINT ["./docker-entrypoint.sh"]
