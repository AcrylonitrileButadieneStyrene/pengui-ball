use leptos::prelude::*;

pub mod easyrpg;

use easyrpg::Player;

#[island]
pub fn Provider(game: String, children: Children) -> impl IntoView {
    provide_context(std::sync::Arc::new(EngineState {
        game,
        easyrpg_player: Player::default(),
        music_volume: RwSignal::new(100),
        sound_volume: RwSignal::new(100),
        muted: RwSignal::new(false),
    }));
    children()
}

pub struct EngineState {
    pub game: String,
    pub easyrpg_player: Player,

    pub music_volume: RwSignal<u8>,
    pub sound_volume: RwSignal<u8>,
    pub muted: RwSignal<bool>,
}
