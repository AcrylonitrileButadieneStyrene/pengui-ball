use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

use crate::sidebar::session::CommandChannel;

mod chat;
mod message;
mod player;

pub use chat::ChatState;
pub use message::Message;
pub use player::Player;

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(Arc::new(State::new()));
    children()
}

pub struct State {
    pub player_count: RwSignal<Option<u32>>,
    pub session_command: CommandChannel,
    pub chat: ChatState,
    pub players: RwSignal<HashMap<String, Arc<Player>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            player_count: RwSignal::new(None),
            session_command: CommandChannel::new(),
            chat: ChatState::default(),
            players: RwSignal::new(HashMap::new()),
        }
    }
}
