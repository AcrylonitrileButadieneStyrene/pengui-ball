use std::sync::Arc;

use common::config::Game;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub current_game_id: Arc<str>,
    pub current_game: Arc<Game>,
    pub all_games: Arc<indexmap::IndexMap<Arc<str>, Arc<Game>>>,
}

pub struct Builder {
    all_games: Arc<indexmap::IndexMap<Arc<str>, Arc<Game>>>,
}

impl Builder {
    pub fn new(config: &common::ServerConfiguration) -> Self {
        Self {
            all_games: config.games.clone(),
        }
    }

    pub fn with_game(self, game_id: Arc<str>) -> Option<Configuration> {
        self.all_games
            .get(&game_id)
            .cloned()
            .map(|current_game| Configuration {
                current_game_id: game_id,
                current_game,
                all_games: self.all_games,
            })
    }
}
