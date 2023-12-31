FROM rust:latest as build

WORKDIR /app

COPY Cargo.* .
COPY src /app/src

RUN \
  DEBIAN_FRONTEND=noninteractive \
  apt-get update &&\
  apt-get -y install ca-certificates tzdata &&\
  cargo build --release

FROM debian:12-slim

WORKDIR /app

RUN \
  DEBIAN_FRONTEND=noninteractive \
  apt-get update &&\
  apt-get -y install openssl &&\
  apt-get clean autoclean &&\
  apt-get autoremove --yes &&\
  rm -rf /var/lib/{apt,dpkg,cache,log}/

COPY --from=build \
  /usr/share/zoneinfo \
  /usr/share/zoneinfo
COPY --from=build \
  /etc/ssl/certs/ca-certificates.crt \
  /etc/ssl/certs/ca-certificates.crt
COPY --from=build \
  /app/target/release/tessellate-bot \
  /usr/bin/tessellate-bot

ENTRYPOINT ["tessellate-bot"]