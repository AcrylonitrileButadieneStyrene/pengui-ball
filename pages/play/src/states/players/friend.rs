use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct Friend {
    pub accepted: bool,
    pub account: bool,
    pub badge: Arc<str>,
    pub game: Arc<str>,
    pub incoming: bool,
    pub lastActive: chrono::DateTime<chrono::Local>,
    pub mapId: String,
    pub medals: [u8; 5],
    pub name: Arc<str>,
    pub online: bool,
    pub prevMapId: String,
    pub rank: u8,
    pub spriteIndex: u8,
    pub spriteName: Arc<str>,
    pub systemName: Arc<str>,
    pub uuid: Arc<str>,
    pub x: i16,
    pub y: i16,
}
