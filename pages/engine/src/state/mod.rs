use leptos::prelude::*;

mod easyrpg_player;
pub use easyrpg_player::EasyRPGPlayer;

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(std::sync::Arc::new(EngineState::default()));

    children()
}

#[derive(Default)]
pub struct EngineState {
    pub easyrpg_player: EasyRPGPlayer,
}
