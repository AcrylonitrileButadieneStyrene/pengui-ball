use std::sync::Arc;

use leptos::prelude::*;

pub mod api;
pub mod chat;
mod config;
pub mod engine;
mod player;

pub use player::Player;

use crate::sidebar::session::SessionState;

#[island]
pub fn Provider(game_id: Arc<str>, children: Children) -> impl IntoView {
    provide_context(Arc::new(PlayState::new(game_id)));
    children()
}

pub struct PlayState {
    pub chat: chat::State,
    pub api: api::State,
    pub engine: engine::State,
    pub session: SessionState,
    pub players: player::State,
    pub config: config::State,
    pub modal: RwSignal<Option<crate::modals::Modals>>,
    pub expeds: RwSignal<Option<crate::modals::expeds::types::Expeds>>,

    pub locations: crate::states::Locations,
}

impl PlayState {
    fn new(game_id: Arc<str>) -> Self {
        let api = api::State::new();

        Self {
            chat: chat::State::new(Signal::derive(move || {
                api.user
                    .read()
                    .as_ref()
                    .map(Result::as_ref)
                    .and_then(Result::ok)
                    .map(|user| user.uuid.clone())
            })),
            session: SessionState::default(),
            players: player::State::default(),
            engine: engine::State::default(),
            api,
            config: config::State::new(&game_id),
            modal: RwSignal::new(None),
            expeds: RwSignal::new(None),

            locations: Arc::new(crate::states::locations::Locations::new(game_id)),
        }
    }
}
