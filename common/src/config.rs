use std::{collections::HashMap, sync::Arc};

#[derive(Debug, serde::Deserialize)]
pub struct ServerConfiguration {
    pub games: Arc<indexmap::IndexMap<Arc<str>, Arc<Game>>>,
    #[serde(skip)]
    pub themes: HashMap<Arc<str>, Vec<Arc<str>>>,
    #[serde(default)]
    pub motd: Option<Arc<str>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub name: Arc<str>,
    #[serde(default)]
    pub permission: PermissionStatus,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Game {}

impl std::hash::Hash for Game {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PermissionStatus {
    Yume1kki,
    Yume2kki,
    #[default]
    Pending,
}
