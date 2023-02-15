# build stage
FROM rust:1.67.1-slim-bullseye AS build-stage

RUN apt update
RUN apt install -y libssl-dev pkg-config

RUN mkdir -p /app/github-notification
RUN mkdir -p /app/github-notification-server

COPY ./github-notification /app/github-notification
COPY ./github-notification-server /app/github-notification-server
WORKDIR /app/github-notification-server
RUN cargo build --release

# runtime stage
FROM debian:bullseye-slim AS runtime-stage

RUN apt update
RUN apt install -y ca-certificates

COPY --from=build-stage /app/github-notification-server/target/release/github-notification-server /app/github-notification-server

ENTRYPOINT ["/app/github-notification-server"]
