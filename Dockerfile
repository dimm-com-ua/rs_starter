FROM rust:1.60 as build

RUN USER=root cargo new --bin dmytro-rs-web
WORKDIR /dmytro-rs-web

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/dmytro_rs_web*
RUN rustup override set nightly
RUN cargo build --release

FROM rust:1.60

COPY --from=build /dmytro-rs-web/target/release/dmytro-rs-web .

CMD ["./dmytro-rs-web"]