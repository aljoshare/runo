FROM cgr.dev/chainguard/rust:latest-dev as build
USER root
RUN apk update && apk add openssl openssl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM cgr.dev/chainguard/wolfi-base
RUN apk update && apk add libgcc
COPY --from=build --chown=nonroot:nonroot /app/target/release/runo /usr/local/bin/runo
CMD ["/usr/local/bin/runo"]