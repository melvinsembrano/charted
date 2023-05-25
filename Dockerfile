# syntax = docker/dockerfile:1

FROM rust:1.69-slim-buster as rust-builder

RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y curl pkg-config libfreetype6-dev libfontconfig1-dev

WORKDIR /rust/charted
RUN curl -sL https://github.com/melvinsembrano/charted/archive/refs/tags/v0.0.2.tar.gz | tar xz --strip-components=1
RUN cargo build --release

RUN cp /rust/charted/target/release/charted /usr/local/bin/charted

ENTRYPOINT ["/usr/local/bin/charted"]
