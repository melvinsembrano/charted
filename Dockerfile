# syntax = docker/dockerfile:1

FROM rust:1.69-slim-buster as rust-builder

RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y curl pkg-config libfreetype6-dev libfontconfig1-dev

WORKDIR /rust/charted
COPY . .

RUN cargo build --release

RUN cp /rust/charted/target/release/charted /usr/local/bin/charted

ENTRYPOINT ["/usr/local/bin/charted"]
