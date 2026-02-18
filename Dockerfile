# from https://hub.docker.com/_/rust/
FROM docker.io/library/rust:latest as builder
WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && apt-get install -y musl-tools clang
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos
RUN cargo install cargo-leptos
ENV LEPTOS_BIN_TARGET_TRIPLE x86_64-unknown-linux-musl
RUN cargo leptos build --release

FROM docker.io/library/nginx:alpine
WORKDIR /app

COPY nginx /etc/nginx
RUN sed -i '/daemon off;/d' /etc/nginx/nginx.conf
RUN sed -i 's|root \.\./public;|root /app/public;|g' /etc/nginx/nginx.conf
COPY config /app/config
COPY --from=builder /usr/src/myapp/target/x86_64-unknown-linux-musl/release/server /app/server
COPY --from=builder /usr/src/myapp/public /app/public

EXPOSE 8080
CMD nginx && /app/server
