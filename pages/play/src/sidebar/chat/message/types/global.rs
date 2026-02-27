use std::sync::Arc;

use leptos::prelude::*;

use crate::{
    state::chat::message::{MessageItem, MessageType},
    states::locations::Location,
};

#[derive(Clone)]
pub struct GlobalMessage {
    pub author: Arc<str>,
    pub location: Option<Location>,
}

impl MessageType for GlobalMessage {
    fn render(&self, message: &MessageItem, state: &crate::state::PlayState) -> AnyView {
        super::remove_if_self(&state.chat, message, &self.author);

        let location = self.location.clone();
        let timestamp = super::timestamp(message.timestamp);
        let uuid = self.author.clone();
        let text = message.text.clone();

        view! {
            <super::Message
                filtered=message.filtered
                header=move || {
                    view! {
                        <crate::sidebar::location::Location location />
                        <span>{timestamp}</span>
                    }
                }
            >
                <super::icons::Megaphone />
                <super::author::Author uuid />
                <span>{text}</span>
            </super::Message>
        }
        .into_any()
    }
}
