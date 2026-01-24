use std::sync::Arc;

use leptos::prelude::*;

use crate::sidebar::session::CommandChannel;

mod chat;
mod engine;
mod message;
mod player;
mod user;

pub use chat::ChatState;
pub use engine::EngineState;
pub use message::{Message, MessageData};
pub use player::{Player, PlayersState};

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(Arc::new(PlayState::new()));
    children()
}

pub struct PlayState {
    pub chat: ChatState,
    pub session_command: CommandChannel,
    pub players: PlayersState,
    pub engine: EngineState,
    pub user: LocalResource<Result<user::User, gloo_net::Error>>,
}

impl PlayState {
    fn new() -> Self {
        Self {
            chat: ChatState::default(),
            session_command: CommandChannel::new(),
            players: PlayersState::default(),
            engine: EngineState::default(),
            user: LocalResource::new(|| async {
                Ok(gloo_net::http::Request::get("api/info")
                    .send()
                    .await?
                    .json()
                    .await
                    .unwrap())
            }),
        }
    }
}
