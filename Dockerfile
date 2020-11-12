FROM rust:1.47.0-alpine3.12 as cargo-build

WORKDIR /usr/src/ticket

RUN apk add --no-cache -U musl-dev openssl-dev

COPY . .

RUN cargo install --path . --root /user/local/bin

FROM alpine:3.12

COPY --from=cargo-build /user/local/bin/ticket /usr/local/bin/ticket

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/ticket"]
