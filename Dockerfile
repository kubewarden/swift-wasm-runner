# build image
FROM rust:1.54-buster as builder

RUN rustup target add x86_64-unknown-linux-musl && \
  apt-get update && \
  apt-get install -y musl-tools

WORKDIR /usr/src/swift-wasm-runner
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --root /usr/local/cargo --path .


# final image
FROM ghcr.io/swiftwasm/swiftwasm-action:5.5
LABEL org.opencontainers.image.source https://github.com/kubewarden/swift-wasm-runner

# Overwrite the wasmer binary, `cartoon test` doesn't have a way to specify which
# Wasm host should be used.
COPY --from=builder /usr/local/cargo/bin/swift-wasm-runner /usr/local/bin/wasmer
