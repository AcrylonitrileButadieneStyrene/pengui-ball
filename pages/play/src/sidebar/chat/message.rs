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

    // not reactive
    let timestamp = message.timestamp.format(
        if message.timestamp.date_naive() < chrono::Local::now().date_naive() {
            "%i:%M %p (%a)"
        } else {
            "%i:%M %p"
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
                <div class=style::author>
                    {name_start} {sender}
                    <img
                        class=style::badge
                        src="https://ynoproject.net/2kki/images/badge/lotus_girl.png"
                    /> {name_end}
                </div>
                <span>{message.content}</span>
            </div>
        </div>
    }
}
