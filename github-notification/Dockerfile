# build stage
FROM rust:1.67.1-slim-bullseye AS build-stage

RUN apt update
RUN apt install -y libssl-dev pkg-config

RUN mkdir -p /app
WORKDIR /app

COPY . /app
RUN cargo build --release

# runtime stage
FROM debian:bullseye-slim AS runtime-stage

RUN apt update
RUN apt install -y ca-certificates

COPY --from=build-stage /app/target/release/github-notification /app/github-notification

ENTRYPOINT ["/app/github-notification"]
