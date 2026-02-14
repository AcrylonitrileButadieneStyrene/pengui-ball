#[derive(serde::Serialize, serde::Deserialize)]
pub enum Message {
    Connect,
    Mute(bool),
    Defocus,
}
