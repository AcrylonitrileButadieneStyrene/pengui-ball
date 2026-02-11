#![feature(lock_value_accessors)]

#[cfg(all(not(feature = "ssr"), not(clippy)))]
compile_error!("Use `cargo leptos serve`");

mod serve;
mod theme_compiler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    serve::run().await;
}
