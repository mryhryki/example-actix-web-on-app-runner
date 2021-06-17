FROM rust:latest AS builder
WORKDIR /usr/local/example-actix-web-on-app-runner/

COPY ./ ./
RUN cargo build --release

EXPOSE 8080
CMD ./target/release/example-actix-web-on-app-runner
