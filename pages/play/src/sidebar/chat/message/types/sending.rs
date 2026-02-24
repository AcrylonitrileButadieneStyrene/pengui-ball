use leptos::prelude::*;

use crate::state::chat::message::{MessageItem, MessageType};

pub struct SendingMessage;

impl MessageType for SendingMessage {
    fn render(&self, message: &MessageItem) -> AnyView {
        let timestamp = super::timestamp(message.timestamp);
        let text = message.text.to_string();

        // todo: this is a hack. it should be replaced with something that
        // properly removes the message from the channel it is in and the
        // primary message list.
        let (dismissed, set_dismissed) = signal(false);
        view! {
            <super::Message
                filtered=message.filtered
                header=move || {
                    view! {
                        <span>Sending...</span>
                        <button class=super::style::dismiss on:click=move |_| set_dismissed(true)>
                            {"\u{2716}"}
                        </button>
                        <span>{timestamp}</span>
                    }
                }
                {..}
                class:hidden=dismissed
                style:order="-1"
            >
                <span class=super::style::sending>{text}</span>
            </super::Message>
        }
        .into_any()
    }
}
