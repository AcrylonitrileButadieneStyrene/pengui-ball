#[derive(serde::Serialize, serde::Deserialize)]
pub enum Message {
    ConnectionStatusUpdated(ConnectionStatus),
    SyncPlayerData(String, u32, bool, String, [u32; 5], u32),
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
