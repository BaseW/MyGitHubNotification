# build stage
FROM rust:1.64.0-slim-bullseye AS build-stage

RUN apt update
RUN apt install -y libssl-dev pkg-config

RUN mkdir -p /app
WORKDIR /app

COPY . /app
RUN cargo build --release

# runtime stage
FROM debian:bullseye-slim

RUN apt update
RUN apt install -y ca-certificates

COPY --from=build-stage /app/target/release/my-github-notification /usr/bin/my-github-notification

ENTRYPOINT ["/usr/bin/my-github-notification"]
