FROM rust:latest AS builder

WORKDIR "/usr/src/"
COPY "./docker/dummy_main.rs" "./src/main.rs"
COPY "./Cargo.toml" "./Cargo.lock" ./
RUN cargo install --path .
COPY "./src" "./src"
RUN cargo build --release

EXPOSE 8080
CMD "/usr/src/target/release/example-actix-web-on-app-runner"

#FROM debian:buster-slim
#COPY --from=builder "/usr/src/target/release/example-actix-web-on-app-runner" "/bin/example-actix-web-on-app-runner"
#EXPOSE 8080
#CMD "/bin/example-actix-web-on-app-runner"
