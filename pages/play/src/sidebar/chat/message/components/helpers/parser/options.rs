use std::sync::Arc;

#[derive(Default)]
pub struct Options {
    pub screenshots: Option<Arc<str>>,
}
