FROM ekidd/rust-musl-builder:nightly-2021-12-23 AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release

FROM alpine:latest
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/app \
    /usr/local/bin/
CMD /usr/local/bin/app
