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

    view! {
        <div
            class=style::message
            style:display=move || {
                if message.filtered.map_or_default(|filter| filter.get()) { "none" } else { "" }
            }
        >
            {match &message.data {
                MessageData::Map { author, text }
                | MessageData::Party { author, text }
                | MessageData::Global { author, text } => {
                    view! {
                        <div class=style::header>
                            {match &message.data {
                                MessageData::Party { .. } | MessageData::Global { .. } => {
                                    Some(view! { <span>Unknown Location</span> })
                                }
                                _ => None,
                            }} <span>{timestamp.to_string()}</span>
                        </div>
                        <div>
                            {match &message.data {
                                MessageData::Party { .. } => Some(super::icons::People().into_any()),
                                MessageData::Global { .. } => {
                                    Some(super::icons::Megaphone().into_any())
                                }
                                _ => None,
                            }} <Author uuid=author.clone() /> <span>{text.to_string()}</span>
                        </div>
                    }
                }
            }}
        </div>
    }
}

#[island]
fn Author(uuid: Arc<str>) -> impl IntoView {
    let state = crate::state();
    let author = state
        .players
        .with_untracked(|players| players.get(&uuid).copied());

    author.map(|player| {
        move || {
            let player = player.get();

            let (name_start, name_end) = if player.account {
                ("[", "]")
            } else {
                ("<", ">")
            };

            let badge = player.badge.as_ref().map(|badge| {
                view! {
                    <img
                        class=style::badge
                        src=format!("https://ynoproject.net/2kki/images/badge/{badge}.png")
                    />
                }
            });

            view! { <div class=style::author>{name_start} {player.name.clone()} {badge} {name_end}</div> }
        }
    }).into_any()
}
