use std::sync::Arc;

use leptos::prelude::*;

pub mod api;
pub mod chat;
pub mod engine;
mod player;
mod user;

pub use chat::{Message, MessageData};
pub use player::Player;

use crate::sidebar::session::SessionState;

#[island]
pub fn Provider(game_id: Arc<str>, children: Children) -> impl IntoView {
    provide_context(Arc::new(PlayState::new(game_id)));
    children()
}

pub struct PlayState {
    pub chat: chat::State,
    pub session: SessionState,
    pub players: player::State,
    pub engine: engine::State,
    pub api: api::State,
    pub modal: RwSignal<Option<crate::modals::Modals>>,

    pub game_id: Arc<str>,
    pub location: RwSignal<Option<(u16, u16, u16)>>,
}

impl PlayState {
    fn new(game_id: Arc<str>) -> Self {
        Self {
            chat: chat::State::default(),
            session: SessionState::default(),
            players: player::State::default(),
            engine: engine::State::default(),
            api: api::State::new(game_id.clone()),
            modal: RwSignal::new(None),
            game_id,
            location: RwSignal::new(None),
        }
    }
}
