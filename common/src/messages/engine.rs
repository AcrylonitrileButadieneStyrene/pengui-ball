#[derive(serde::Serialize, serde::Deserialize)]
pub enum Message {
    Connect,
    Mute(bool),
    Focus(bool),
    SetSave(usize, Vec<u8>),
    GetSave(usize),
    DeleteSave(usize),
}
