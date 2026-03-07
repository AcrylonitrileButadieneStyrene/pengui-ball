use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Friend {
    pub accepted: bool,
    pub account: bool,
    pub badge: Arc<str>,
    pub game: Arc<str>,
    pub incoming: bool,
    #[serde(rename = "lastActive")]
    pub last_active: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "mapId")]
    pub map_id: String,
    pub medals: [u8; 5],
    pub name: Arc<str>,
    pub online: bool,
    #[serde(rename = "prevMapId")]
    pub prev_map_id: Option<String>,
    pub rank: u8,
    #[serde(rename = "spriteIndex")]
    pub sprite_index: u8,
    #[serde(rename = "spriteName")]
    pub sprite_name: Arc<str>,
    #[serde(rename = "systemName")]
    pub system_name: Arc<str>,
    pub uuid: Arc<str>,
    pub x: i16,
    pub y: i16,
}
