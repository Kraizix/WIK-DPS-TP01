# docker build -t wik-dps-tp02-rust-multi -f multi-stage.dockerfile .
FROM rust:1.64.0-alpine AS builder

WORKDIR /build

RUN cargo new --bin wik_dps_tp01
WORKDIR /build/wik_dps_tp01

COPY Cargo.* ./
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/wik_dps_tp01*

COPY ./src ./src
RUN cargo build --release

RUN addgroup -S appgroup && adduser -S appuser -G appgroup

USER appuser

FROM scratch
COPY --from=builder /build/wik_dps_tp01/target/release/wik_dps_tp01 /app

CMD ["/app"]