#![allow(clippy::unused_async)]
#![allow(non_snake_case)]

mod app;
mod config;
mod macros;
mod pages;

pub use app::{App, shell};
pub use config::Config;
