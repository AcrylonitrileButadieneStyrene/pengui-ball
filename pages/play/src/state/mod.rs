use std::sync::Arc;

use leptos::prelude::*;

use crate::sidebar::session::CommandChannel;

mod chat;
mod engine;
mod message;
mod player;

pub use chat::ChatState;
pub use engine::EngineState;
pub use message::{Message, MessageData};
pub use player::{Player, PlayersState};

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(Arc::new(State::default()));
    children()
}

pub struct State {
    pub chat: ChatState,
    pub session_command: CommandChannel,
    pub players: PlayersState,
    pub engine: EngineState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            chat: ChatState::default(),
            session_command: CommandChannel::new(),
            players: PlayersState::default(),
            engine: EngineState::default(),
        }
    }
}
