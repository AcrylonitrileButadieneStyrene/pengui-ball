#![feature(nonpoison_mutex)]
#![feature(sync_nonpoison)]
#![feature(iter_intersperse)]

mod component;
mod resolver;

pub use component::Location;
pub use resolver::{LocationResolved, Resolver};

#[derive(Clone, Debug)]
pub struct Location {
    pub game: std::sync::Arc<str>,
    pub map: u16,
    pub previous: Option<u16>,
    pub x: i16,
    pub y: i16,
}
