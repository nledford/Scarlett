#-----------------------------------------------------------------------------------------------------------------------
# Build Stage
#-----------------------------------------------------------------------------------------------------------------------

# Using musl-builder image since it has a statically linked openssl library included
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

# Switch to root user so we can install packages
USER root

# Install libheif package for working with '.heic' images
RUN apt-get update
RUN apt-get install libheif-dev -y

# Switch back to rust user to build application
USER rust

# Build dummy application so that dependencies won't have to be rebuilt on subsequent runs
WORKDIR ./
RUN USER=root cargo new scarlett-server
WORKDIR ./scarlett-server
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Build the real application
COPY src ./src
RUN cargo build --release

#-----------------------------------------------------------------------------------------------------------------------
# Final Stage
#-----------------------------------------------------------------------------------------------------------------------

FROM alpine:latest

# Install ca-certificates for openssl
# and libheif for working with '.heic' images
RUN apk --no-cache add ca-certificates libheif

COPY --from=builder \
        /home/rust/src/scarlett-server/target/x86_64-unknown-linux-musl/release/scarlett-server \
        /usr/local/bin

CMD /usr/local/bin/scarlett-server