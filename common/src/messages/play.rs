#[derive(serde::Serialize, serde::Deserialize)]
pub enum Message {
    EngineLoaded,
    ConnectionStatusUpdated(ConnectionStatus),
    PlayerSync(PlayerSyncData),
    PlayerConnect(PlayerConnectData),
    RegainFocus(bool),
    PlayerTeleported(u16, u16, u16),
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
    pub rank: u32,
    pub account: bool,
    pub badge: String,
    pub medals: [u32; 5],
    pub id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlayerConnectData {
    pub system: String,
    pub name: String,
    pub id: i32,
}
