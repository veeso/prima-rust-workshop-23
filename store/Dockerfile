FROM rust:1.68.0-slim-buster

RUN cargo install sqlx-cli

USER root

ENTRYPOINT ["./docker-entrypoint.sh"]
