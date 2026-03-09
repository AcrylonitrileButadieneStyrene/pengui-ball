use std::sync::Arc;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Friend {
    pub name: Arc<str>,
    pub badge: Arc<str>,
    #[serde(rename = "systemName")]
    pub system: Arc<str>,
    #[serde(rename = "spriteName")]
    pub sprite_name: Arc<str>,
    #[serde(rename = "spriteIndex")]
    pub sprite_index: u8,
    pub account: bool,
    pub medals: [u8; 5],
    pub rank: u8,
    pub uuid: Arc<str>,
    pub game: Arc<str>,
    #[serde(rename = "mapId")]
    pub map_id: Arc<str>,
    #[serde(rename = "prevMapId")]
    pub prev_map_id: Option<Arc<str>>,
    #[serde(rename = "lastActive")]
    pub last_active: chrono::DateTime<chrono::Utc>,
    pub incoming: bool,
    pub accepted: bool,
    pub online: bool,
    pub x: i16,
    pub y: i16,
}
