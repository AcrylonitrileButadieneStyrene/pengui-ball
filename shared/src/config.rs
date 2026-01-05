use leptos::prelude::*;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
}

#[server]
pub async fn get_games() -> Result<Vec<Game>, ServerFnError> {
    Ok(use_context::<std::sync::Arc<Config>>()
        .unwrap()
        .games
        .clone())
}
