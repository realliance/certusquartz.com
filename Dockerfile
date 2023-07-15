FROM rust:buster as wasm_builder

WORKDIR /build

# Install Deps
RUN apt-get update && apt-get install -y binaryen && cargo install just

ADD Justfile .

# Setup Global Deps
RUN just install-global-deps

# Add Files
ADD game game

ADD Cargo.toml .
ADD Cargo.lock .
ADD rust-toolchain.toml .

# Build
RUN just build-client

FROM node:current as web_builder

WORKDIR /build

# Add Web Files
ADD web web
ADD package.json .
ADD yarn.lock .
COPY --from=wasm_builder /build/web/src/wasm /build/web/src/wasm
ADD game/assets /build/web/public/assets

# Build
RUN yarn install && yarn workspace web build

FROM docker.io/nginx:1-alpine

COPY --from=web_builder /build/web/dist /usr/share/nginx/html
ADD nginx.conf /etc/nginx/nginx.conf
