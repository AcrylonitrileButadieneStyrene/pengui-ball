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
        <Show when=move || {
            !message.filtered.map_or_default(|filter| filter.get())
        }>
            {match &message.data {
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
            }}
        </Show>
    }
}

#[island]
fn Author(uuid: Arc<str>) -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::state::State>>().unwrap();
    let author = state
        .players
        .with_untracked(|players| players.get(&uuid).cloned());

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
    })
}
