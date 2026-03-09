use std::sync::Arc;

use leptos::prelude::*;

use crate::{
    sidebar::chat::message::{Message, components::sending::SendingMessage},
    state::chat::message::{MessageComponent, MessageItem},
    states::players::player::PlayerStoreFields,
};

pub trait ChatMessageComponent {
    fn author(&self) -> Arc<str>;

    fn header(&self) -> AnyView {
        ().into_any()
    }

    fn icon(&self) -> AnyView {
        ().into_any()
    }
}

impl<T: ChatMessageComponent + Send + Sync> MessageComponent for T {
    fn render(&self, message: &MessageItem, state: &crate::state::PlayState) -> AnyView {
        let author = self.author();
        let is_self = state
            .chat
            .my_id
            .get_untracked()
            .or(state.players.local.uuid().get_untracked())
            .is_some_and(|id| id == author);
        if is_self {
            state.chat.channel::<SendingMessage>().remove(message);
        }

        let pinged = has_ping(state, &message.text);
        if pinged && let Some(mention_audio) = &*state.chat.mention_audio {
            let _ = mention_audio.play().ok();
        }

        let header = self.header();
        let timestamp = super::timestamp(message.timestamp);

        let icon = self.icon();
        let text = message.text.clone();

        view! {
            <Message
                filtered=message.filtered
                header=move || {
                    view! {
                        {header}
                        <span>{timestamp}</span>
                    }
                }
                {..}
                class:highlight=pinged
            >
                {icon}
                <super::super::author::Author uuid=author />
                <span>{text}</span>
            </Message>
        }
        .into_any()
    }
}

fn has_ping(state: &crate::state::PlayState, message: &str) -> bool {
    let Some(username) = state
        .api
        .user
        .read()
        .as_ref()
        .map(Result::as_ref)
        .map(Result::ok)
        .flatten()
        .map(|user| user.name.clone())
        .filter(|name| !name.is_empty())
        .or_else(|| state.chat.guest_name.get())
    else {
        return false;
    };

    message
        .split(' ')
        .filter(|part| part.starts_with('@'))
        .any(|part| part[1..].to_lowercase() == username.to_lowercase())
}
