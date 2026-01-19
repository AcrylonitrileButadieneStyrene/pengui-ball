use std::sync::Arc;

use leptos::prelude::*;

use crate::state::{Message, Player};

stylance::import_style!(pub style, "message.module.css");

#[component]
pub fn ChatMessage(message: Message, author: Option<Arc<Player>>) -> impl IntoView {
    let account = author.as_ref().map_or_default(|player| player.account);
    let sender = author
        .as_ref()
        .map_or_else(|| message.author, |player| player.name.clone());

    view! {
        <div class=style::message>
            <span class=style::author class:user=account>
                {sender}
            </span>
            <span>{message.content}</span>
        </div>
    }
}
