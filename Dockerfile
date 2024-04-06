FROM rust:1.76.0-alpine3.19 as build

RUN apk add g++ pkgconfig openssl-dev alpine-sdk

WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src; echo 'fn main() {}' > src/main.rs
RUN cargo install --locked --path .
RUN rm -rf src;
COPY src src
RUN cargo build --release

FROM alpine:3.19.1 as run
WORKDIR /app
COPY --from=build /app/target/release/tairitsu-rs .
ENTRYPOINT /app/tairitsu-rs