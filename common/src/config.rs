#[derive(Debug, serde::Deserialize)]
pub struct ServerConfiguration {
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
}
