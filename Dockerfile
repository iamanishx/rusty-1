FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM alpine:3.19
RUN apk add --no-cache ca-certificates
COPY --from=builder /usr/src/app/target/release/rust-1 /usr/local/bin/proxy
COPY envFiles/.env.example /usr/local/bin/.env
WORKDIR /usr/local/bin
CMD ["proxy"]