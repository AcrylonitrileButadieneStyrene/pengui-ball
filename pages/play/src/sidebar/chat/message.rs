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
    let badge = author.as_ref().and_then(|player| {
        player.badge.as_ref().map(|badge| {
            view! {
                <img
                    class=style::badge
                    src=format!("https://ynoproject.net/2kki/images/badge/{badge}.png")
                />
            }
        })
    });

    // not reactive
    let timestamp = message.timestamp.format(
        if message.timestamp.date_naive() < chrono::Local::now().date_naive() {
            "%l:%M %p (%a)"
        } else {
            "%l:%M %p"
        },
    );

    let (name_start, name_end) = if account { ("[", "]") } else { ("<", ">") };

    view! {
        <div class=style::message>
            <div class=style::header>
                <span>Unknown Location</span>
                {timestamp.to_string()}
            </div>
            <div>
                <div class=style::author>{name_start} {sender} {badge} {name_end}</div>
                <span>{message.content}</span>
            </div>
        </div>
    }
}
