#-----------------------------------------------------------------------------------------------------------------------
# Build Stage
#-----------------------------------------------------------------------------------------------------------------------

# Using musl-builder image since it has a statically linked openssl library included
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

# build dummy application so that dependencies won't have to be rebuilt on subsequent runs
WORKDIR ./
RUN USER=root cargo new scarlett-server
WORKDIR ./scarlett-server
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# build the real application
COPY src ./src
#RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN cargo build --release

#-----------------------------------------------------------------------------------------------------------------------
# Final Stage
#-----------------------------------------------------------------------------------------------------------------------

FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder \
        /home/rust/src/scarlett-server/target/x86_64-unknown-linux-musl/release/scarlett-server \
        /usr/local/bin

CMD /usr/local/bin/scarlett-server