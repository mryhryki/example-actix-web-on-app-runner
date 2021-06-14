FROM rust:latest

WORKDIR "/usr/src/"
COPY "./dummy/main.rs" "./src/main.rs"
COPY "./Cargo.toml" "./Cargo.lock" ./
RUN cargo install --path .

COPY "./src" "./src"
RUN cargo build --release

EXPOSE 8080
CMD "./target/release/example-actix-web-on-app-runner"
