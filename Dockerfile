FROM rust:1.64.0-alpine

RUN USER=root cargo new --bin WIK_DPS_TP01
WORKDIR /WIK_DPS_TP01

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/wik_dps_tp01*
RUN cargo build --release

RUN addgroup -S appgroup && adduser -S appuser -G appgroup

USER appuser

CMD ["./target/release/wik_dps_tp01"]