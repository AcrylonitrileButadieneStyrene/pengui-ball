use std::sync::Arc;

use leptos::prelude::*;

use super::super::Message;
use crate::state::chat::message::{MessageItem, MessageType};

pub struct MapMessage {
    pub author: Arc<str>,
}

impl MessageType for MapMessage {
    fn on_add(&self, message: &MessageItem, chat: &crate::state::chat::State) {
        super::remove_if_self(chat, message, &self.author);
    }

    fn render(&self, message: &MessageItem) -> AnyView {
        let timestamp = super::timestamp(message.timestamp);
        let uuid = self.author.clone();
        let text = message.text.to_string();

        view! {
            <Message
                filtered=message.filtered
                header=move || {
                    view! { <span>{timestamp}</span> }
                }
            >
                <super::super::author::Author uuid />
                <span>{text}</span>
            </Message>
        }
        .into_any()
    }
}
