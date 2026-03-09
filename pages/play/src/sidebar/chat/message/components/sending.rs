use leptos::prelude::*;

use crate::state::chat::message::{MessageComponent, MessageItem};

pub struct SendingMessage;

impl MessageComponent for SendingMessage {
    fn render(&self, message: &MessageItem, state: &crate::state::PlayState) -> AnyView {
        let timestamp = super::timestamp(message.timestamp);
        let channel = state.chat.channel::<Self>();
        let text = message.text.clone();

        let remove = {
            let text = message.text.clone();
            move |_| channel.remove_by_text(&text)
        };

        view! {
            <super::Message
                filtered=message.filtered
                header=move || {
                    view! {
                        <span>Sending...</span>
                        <button class=super::style::dismiss on:click=remove>
                            {"\u{2716}"}
                        </button>
                        <span>{timestamp}</span>
                    }
                }
                {..}
                style:order="-1"
            >
                <span class=super::style::sending>{text}</span>
            </super::Message>
        }
        .into_any()
    }
}
