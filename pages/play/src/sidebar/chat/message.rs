use std::sync::Arc;

use leptos::prelude::*;

use crate::state::{Message, MessageData};

stylance::import_style!(pub style, "message.module.css");

#[component]
pub fn ChatMessage(message: Message) -> impl IntoView {
    // not reactive
    let timestamp = message.timestamp.format(
        if message.timestamp.date_naive() < chrono::Local::now().date_naive() {
            "%l:%M %p (%a)"
        } else {
            "%l:%M %p"
        },
    );

    match &message.data {
        MessageData::Map { author, text }
        | MessageData::Party { author, text }
        | MessageData::Global { author, text } => {
            view! {
                <div class=style::message>
                    <div class=style::header>
                        <span>Unknown Location</span>
                        {timestamp.to_string()}
                    </div>
                    <div>
                        <Author uuid=author.clone() />
                        <span>{text.to_string()}</span>
                    </div>
                </div>
            }
        }
    }
}

#[island]
fn Author(uuid: Arc<str>) -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::state::State>>().unwrap();
    let author = state
        .players
        .with_untracked(|players| players.get(&uuid).cloned());
    let (account, sender, badge) = author.as_ref().map_or_default(|player| {
        (
            player.account,
            player.name.clone(),
            player.badge.as_ref().map(|badge| {
                view! {
                    <img
                        class=style::badge
                        src=format!("https://ynoproject.net/2kki/images/badge/{badge}.png")
                    />
                }
            }),
        )
    });
    let (name_start, name_end) = if account { ("[", "]") } else { ("<", ">") };

    view! {
        <div class=style::author>{name_start} {sender} {badge} {name_end}</div>
    }
}
