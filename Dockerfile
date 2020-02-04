# Dockerfile based on this tutorial:
# https://shaneutt.com/blog/rust-fast-small-docker-image-builds/

#------------------------------------------------------------------------------
# Cargo build stage
#------------------------------------------------------------------------------

FROM rust:1.41 as cargo-build

RUN apt-get update
RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/scarlett-server

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/scarlett-server*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

#------------------------------------------------------------------------------
# Final Stage
#------------------------------------------------------------------------------

FROM alpine:3

# Create 'scarlett-server' user
RUN addgroup -g 1000 scarlett-server
RUN adduser -D -s /bin/sh -u 1000 -G scarlett-server scarlett-server

# Copy from cargo builder
WORKDIR /home/scarlett-server/bin

COPY --from=cargo-build /usr/src/scarlett-server/target/x86_64-unknown-linux-musl/release/scarlett-server .

RUN chown scarlett-server:scarlett-server scarlett-server

USER scarlett-server

CMD ["./scarlett-server"]
