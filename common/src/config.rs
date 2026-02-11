use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct ServerConfiguration {
    pub games: Vec<Game>,
    #[serde(skip)]
    pub menu_themes: HashMap<String, MenuTheme>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct MenuTheme {
    pub game: String,
    pub gradient: String,
}
