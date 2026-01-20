use leptos::prelude::*;

pub mod easyrpg;

use easyrpg::Player;

#[island]
pub fn Provider(game: String, children: Children) -> impl IntoView {
    let state = EngineState {
        game,
        easyrpg_player: Player::default(),
    };

    provide_context(std::sync::Arc::new(state));

    children()
}

pub struct EngineState {
    pub game: String,
    pub easyrpg_player: Player,
}
