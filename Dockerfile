# from https://hub.docker.com/_/rust/
FROM docker.io/library/rust:alpine AS builder
WORKDIR /usr/src/myapp

RUN apk add --no-cache curl
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo binstall cargo-leptos
ENV LEPTOS_BIN_TARGET_TRIPLE x86_64-unknown-linux-musl
COPY . .
RUN --mount=type=cache,target=/usr/src/myapp/target \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
	cargo leptos build --release && \
	cp /usr/src/myapp/target/x86_64-unknown-linux-musl/release/server /bin/server

FROM docker.io/library/nginx:alpine
WORKDIR /app

COPY nginx /etc/nginx
RUN sed -i '/daemon off;/d' /etc/nginx/nginx.conf
RUN sed -i 's|root \.\./public;|root /app/public;|g' /etc/nginx/nginx.conf
COPY config /app/config
VOLUME [ "app/config" ]
COPY --from=builder /bin/server /app/
COPY --from=builder /usr/src/myapp/public /app/public

EXPOSE 8080
CMD nginx && /app/server
