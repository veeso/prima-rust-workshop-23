FROM rust:1.68.0-slim-bullseye

RUN apt update && \
  apt install -y unzip pkg-config openssl libssl-dev curl && \
  cargo install cargo-make

RUN PROTOC_VERSION=$(curl -s "https://api.github.com/repos/protocolbuffers/protobuf/releases/latest" | grep -Po '"tag_name": "v\K[0-9.]+') && \
  curl -Lo protoc.zip "https://github.com/protocolbuffers/protobuf/releases/latest/download/protoc-${PROTOC_VERSION}-linux-x86_64.zip" && \
  unzip -q protoc.zip bin/protoc -d /usr/local && \
  chmod a+x /usr/local/bin/protoc

COPY . /app/user-api
WORKDIR /app/user-api

USER root

ENTRYPOINT ["./docker-entrypoint.sh"]
