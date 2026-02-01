use std::sync::Arc;

use leptos::prelude::*;

pub mod chat;
pub mod engine;
mod player;
mod user;

pub use chat::{Message, MessageData};
pub use player::Player;

use crate::sidebar::session::SessionState;

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(Arc::new(PlayState::new()));
    children()
}

pub struct PlayState {
    pub chat: chat::State,
    pub session: SessionState,
    pub players: player::State,
    pub engine: engine::State,
    pub user: LocalResource<Option<user::User>>,
    pub modal: RwSignal<Option<crate::modals::Modals>>,
}

impl PlayState {
    fn new() -> Self {
        Self {
            chat: chat::State::default(),
            session: SessionState::default(),
            players: player::State::default(),
            engine: engine::State::default(),
            user: LocalResource::new(|| async {
                gloo_net::http::Request::get("api/info")
                    .send()
                    .await
                    .ok()?
                    .json()
                    .await
                    .ok()?
            }),
            modal: RwSignal::new(None),
        }
    }
}
