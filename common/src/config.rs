use std::sync::Arc;

#[derive(Debug, serde::Deserialize)]
pub struct ServerConfiguration {
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: Arc<str>,
    pub name: String,
    #[serde(default)]
    pub permission: PermissionStatus,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Game {}

impl std::hash::Hash for Game {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PermissionStatus {
    Yume1kki,
    Yume2kki,
    #[default]
    Pending,
}
