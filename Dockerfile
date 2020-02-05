#-----------------------------------------------------------------------------------------------------------------------
# Cargo build stage
#-----------------------------------------------------------------------------------------------------------------------

ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

# Add source code
ADD --chown=rust:rust . ./

# build release version of application
RUN cargo build --release

#-----------------------------------------------------------------------------------------------------------------------
# Final Stage
#-----------------------------------------------------------------------------------------------------------------------

FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder \
        /home/rust/src/target/x86_64-unknown-linux-musl/release/scarlett-server \
        /usr/local/bin

CMD /usr/local/bin/scarlett-server