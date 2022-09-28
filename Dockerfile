# build stage
FROM rust:1.64.0-slim-bullseye AS build-stage

RUN apt update
RUN apt install -y libssl-dev pkg-config

RUN cargo install sccache
RUN export RUSTC_WRAPPER=`which sccache`

RUN mkdir -p /app
WORKDIR /app

COPY . /app
RUN mkdir /usr/bin/target
RUN cargo build --release --target-dir /usr/bin/target

# runtime stage
FROM debian:bullseye-slim AS runtime-stage

RUN apt update
RUN apt install -y ca-certificates

COPY --from=build-stage /usr/bin/target/release/my-github-notification /usr/bin/my-github-notification

ENTRYPOINT ["/usr/bin/my-github-notification"]
