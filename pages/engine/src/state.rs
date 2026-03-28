use std::sync::Arc;

use leptos::prelude::*;

use crate::easyrpg::state::Player;

#[island]
pub fn Provider(game: Arc<str>, children: Children) -> impl IntoView {
    provide_context(std::sync::Arc::new(EngineState {
        game,
        easyrpg_player: Player::default(),
        music_volume: RwSignal::new(100),
        sound_volume: RwSignal::new(100),
        muted: RwSignal::new(false),
        defocus_timeout: RwSignal::new(None),
    }));
    children()
}

pub struct EngineState {
    pub game: Arc<str>,
    pub easyrpg_player: Player,

    pub music_volume: RwSignal<u8>,
    pub sound_volume: RwSignal<u8>,
    pub muted: RwSignal<bool>,
    pub defocus_timeout: RwSignal<Option<TimeoutHandle>>,
}
