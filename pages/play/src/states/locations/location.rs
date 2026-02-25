use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Location {
    pub game: Arc<str>,
    pub map: u16,
    pub previous: Option<u16>,
    pub x: i16,
    pub y: i16,
}
