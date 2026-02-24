pub mod global;
pub mod map;
pub mod party;
pub mod sending;

use crate::{
    sidebar::chat::message::types::sending::SendingMessage, state::chat::message::MessageItem,
};

#[allow(clippy::wildcard_imports)]
use super::*;

fn timestamp(timestamp: chrono::DateTime<chrono::Local>) -> String {
    let format = if timestamp.date_naive() < chrono::Local::now().date_naive() {
        "%l:%M %p (%a)"
    } else {
        "%l:%M %p"
    };
    timestamp.format(format).to_string()
}

fn remove_if_self(chat: &crate::state::chat::State, message: &MessageItem, author: &str) {
    let is_self = chat
        .my_id
        .read_untracked()
        .as_ref()
        .is_some_and(|id| &**id == author);
    if is_self {
        chat.channel::<SendingMessage>().remove(message);
    }
}
