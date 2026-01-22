use std::sync::Arc;

#[derive(Clone)]
pub struct Player {
    pub name: Option<Arc<str>>,
    pub system: Option<Arc<str>>,
    pub rank: u32,
    pub account: bool,
    pub badge: Option<Arc<str>>,
    pub medals: [u32; 5],
}
