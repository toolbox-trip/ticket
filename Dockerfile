FROM rust:1.47.0-alpine3.12 as cargo-build

WORKDIR /usr/src/ticket

COPY . .

RUN cargo build --release

RUN cargo install --path .

FROM alpine:3.12

COPY --from=cargo-build /usr/local/cargo/bin/ticket /usr/local/bin/ticket

EXPOSE 8080

ENTRYPOINT ["./ticket"]
