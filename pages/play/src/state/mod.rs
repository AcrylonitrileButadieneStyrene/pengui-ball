use std::sync::Arc;

use leptos::prelude::*;

pub mod api;
pub mod chat;
mod config;
pub mod engine;

use crate::sidebar::session::SessionState;

#[island]
pub fn Provider(game_id: Arc<str>, children: Children) -> impl IntoView {
    provide_context::<crate::State>(PlayState::new(game_id).into());
    children()
}

pub struct PlayState {
    pub chat: chat::State,
    pub api: api::State,
    pub engine: engine::State,
    pub session: SessionState,
    pub config: config::State,
    pub modal: RwSignal<Option<crate::modals::Modals>>,
    pub expeds: RwSignal<Option<crate::modals::expeds::types::Expeds>>,

    pub badges: crate::states::Badges,
    pub players: crate::states::Players,
    pub locations: crate::states::Locations,
    pub interfaces: crate::states::Interfaces,
}

impl PlayState {
    fn new(game_id: Arc<str>) -> Self {
        let api = api::State::new(&game_id);

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
            engine: engine::State::default(),
            api,
            config: config::State::new(&game_id),
            modal: RwSignal::new(None),
            expeds: RwSignal::new(None),

            badges: Arc::new(crate::states::badges::Badges::new(&game_id)),
            players: Arc::new(crate::states::players::Players::new()),
            locations: Arc::new(crate::states::locations::Locations::new(game_id)),
            interfaces: crate::states::Interfaces::new(),
        }
    }
}

impl From<PlayState> for crate::State {
    fn from(value: PlayState) -> Self {
        Box::leak(Box::new(value))
    }
}
