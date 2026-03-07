use std::sync::Arc;

#[derive(Clone, Default, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Player {
    pub name: Option<Arc<str>>,
    #[serde(rename = "systemName")]
    pub system: Option<Arc<str>>,
    pub rank: u8,
    pub account: bool,
    pub badge: Option<Arc<str>>,
    pub medals: [u8; 5],
}
