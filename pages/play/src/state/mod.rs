use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

use crate::sidebar::session::CommandChannel;

mod chat;
mod engine;
mod message;
mod player;

pub use chat::ChatState;
pub use engine::EngineState;
pub use message::{Message, MessageData};
pub use player::Player;

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(Arc::new(State::default()));
    children()
}

pub struct State {
    pub chat: ChatState,
    pub session_command: CommandChannel,
    pub players: RwSignal<HashMap<Arc<str>, Player>>,
    pub player_count: RwSignal<Option<u32>>,
    pub uuids: RwSignal<HashMap<u32, Arc<str>>>,
    pub engine: EngineState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            chat: ChatState::default(),
            session_command: CommandChannel::new(),
            players: RwSignal::new(HashMap::new()),
            player_count: RwSignal::new(None),
            uuids: RwSignal::new(HashMap::default()),
            engine: EngineState::default(),
        }
    }
}
