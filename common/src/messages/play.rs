use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Message {
    EngineLoaded,
    ConnectionStatusUpdated(ConnectionStatus),
    PlayerSync(PlayerSyncData),
    PlayerConnect(PlayerConnectData),
    PlayerDisconnect(i32),
    PlayerTeleported(u16, i16, i16),
    PlayerSpriteUpdated(i32, String, u8),
    RegainFocus(bool),
    SaveData(usize, SaveFile),
    SaveTimestamps(Box<[Option<String>; 15]>),
    RoomSwitch,
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConnectionStatus {
    /// 0
    Disconnected,
    /// 1
    Connected,
    /// 2
    Connecting,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlayerSyncData {
    pub uuid: String,
    pub rank: u8,
    pub account: bool,
    pub badge: String,
    pub medals: [u8; 5],
    pub id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlayerConnectData {
    pub system: String,
    pub name: String,
    pub id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveFile {
    pub timestamp: String,
    pub contents: Arc<[u8]>,
}
