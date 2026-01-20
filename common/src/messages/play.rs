#[derive(serde::Serialize, serde::Deserialize)]
pub enum Message {
    ConnectionStatusUpdated(ConnectionStatus),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ConnectionStatus {
    /// 0
    Disconnected,
    /// 1
    Connected,
    /// 2
    Connecting,
}
