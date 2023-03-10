FROM rust:1.68.0-slim-buster

RUN apt update && \
  apt install -y python3 && \
  pip3 install requests psycopg2

USER root

ENTRYPOINT ["./docker-entrypoint.sh"]
