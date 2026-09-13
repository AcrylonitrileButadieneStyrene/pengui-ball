use std::sync::Arc;

use leptos::prelude::*;

pub mod api;
pub mod chat;
pub mod config;
pub mod engine;
mod settings;

use crate::sidebar::session::SessionState;

#[island]
pub fn Provider(
    game_id: Arc<str>,
    config: config::Configuration,
    children: Children,
) -> impl IntoView {
    provide_context::<crate::State>(PlayState::new(game_id, config).into());
    children()
}

pub struct PlayState {
    pub api: api::State,
    pub chat: chat::State,
    pub config: config::Configuration,
    pub engine: engine::State,
    pub expeds: RwSignal<Option<crate::modals::expeds::types::Expeds>>,
    pub session: SessionState,
    pub settings: settings::State,
    pub modal: RwSignal<Option<crate::modals::Modals>>,

    pub badges: crate::states::Badges,
    pub players: crate::states::Players,
    pub locations: crate::states::Locations,
    pub interfaces: crate::states::Interfaces,
}

impl PlayState {
    fn new(game_id: Arc<str>, config: config::Configuration) -> Self {
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
            api,
            config,
            engine: engine::State::default(),
            expeds: RwSignal::new(None),
            session: SessionState::default(),
            settings: settings::State::new(&game_id),
            modal: RwSignal::new(None),

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
