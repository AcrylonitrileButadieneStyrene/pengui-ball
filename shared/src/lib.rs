#![allow(clippy::unused_async)]

mod app;
mod config;
mod macros;
mod pages;

pub use app::{App, shell};
pub use config::Config;
