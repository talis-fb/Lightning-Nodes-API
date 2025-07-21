FROM rust:1.88-alpine AS builder

RUN apk update && apk add --no-cache ca-certificates perl musl-dev openssl-dev libc-dev build-base pkgconfig

WORKDIR /app

RUN cargo init
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

RUN rm src/*.rs
RUN rm ./target/release/deps/bipa*

COPY src/ src/

ARG FEATURES=""

RUN cargo build --release --features "$FEATURES"

FROM alpine:latest AS runner

RUN apk add --no-cache ca-certificates
WORKDIR /app

COPY --from=builder /app/target/release/bipa /app/bipa

CMD ["./bipa"]
