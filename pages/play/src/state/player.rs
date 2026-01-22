use std::sync::Arc;

pub struct Player {
    pub name: Arc<str>,
    pub system: Arc<str>,
    pub rank: u32,
    pub account: bool,
    pub badge: Option<Arc<str>>,
    pub medals: [u32; 5],
}
