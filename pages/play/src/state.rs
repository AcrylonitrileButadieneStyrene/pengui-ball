use leptos::prelude::*;

use crate::sidebar::{ChatState, session::CommandChannel};

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(std::sync::Arc::new(State::new()));
    children()
}

pub struct State {
    pub player_count: RwSignal<Option<u32>>,
    pub session_command: CommandChannel,
    pub chat: ChatState,
}

impl State {
    pub fn new() -> Self {
        Self {
            player_count: RwSignal::new(None),
            session_command: CommandChannel::new(),
            chat: ChatState::default(),
        }
    }
}
