# syntax=docker/dockerfile:1

FROM rust:1.80 AS builder
WORKDIR /home/b1nar1us/dev/ruffee
COPY  . .
RUN cargo install --path .

FROM debian:bullseye
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
COPY --from=builder /home/b1nar1us/dev/ruffee /usr/local/bin/ruffee
CMD ["ruffee"]
