use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Message {
    Connect,
    Mute(bool),
    Focus(bool),
    SetSave(usize, Arc<[u8]>, Option<chrono::DateTime<chrono::Utc>>),
    GetSave(usize),
    DeleteSave(usize),
    GetSaveTimestamps,
    SetMusicVolume(u8),
    SetSoundVolume(u8),
    PressKey(u8, bool),
}
