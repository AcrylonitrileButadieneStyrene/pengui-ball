use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Message {
    Connect,
    Mute(bool),
    Focus(bool),
    SetSave(usize, Arc<[u8]>),
    GetSave(usize),
    DeleteSave(usize),
}
