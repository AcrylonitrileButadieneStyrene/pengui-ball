use std::sync::Arc;

use leptos::prelude::*;

mod chat;
mod engine;
mod message;
mod player;
mod user;

pub use chat::ChatState;
pub use engine::EngineState;
pub use message::{Message, MessageData};
pub use player::{Player, PlayersState};

use crate::sidebar::session::SessionState;

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(Arc::new(PlayState::new()));
    children()
}

pub struct PlayState {
    pub chat: ChatState,
    pub session: SessionState,
    pub players: PlayersState,
    pub engine: EngineState,
    pub user: LocalResource<Option<user::User>>,
    pub modal: RwSignal<Option<crate::modals::Modals>>,
}

impl PlayState {
    fn new() -> Self {
        Self {
            chat: ChatState::default(),
            session: SessionState::default(),
            players: PlayersState::default(),
            engine: EngineState::default(),
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
