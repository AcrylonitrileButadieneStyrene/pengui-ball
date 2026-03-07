use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Friend {
    #[serde(flatten)]
    pub player: super::player::Player,
    pub uuid: Arc<str>,
    pub game: Arc<str>,
    #[serde(rename = "mapId")]
    pub map_id: Arc<str>,
    #[serde(rename = "prevMapId")]
    pub prev_map_id: Option<Arc<str>>,
    #[serde(rename = "lastActive")]
    pub last_active: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "spriteName")]
    pub sprite_name: Arc<str>,
    #[serde(rename = "spriteIndex")]
    pub sprite_index: u8,
    pub incoming: bool,
    pub accepted: bool,
    pub online: bool,
    pub x: i16,
    pub y: i16,
}
