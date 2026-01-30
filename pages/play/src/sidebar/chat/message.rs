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
                            }}
                            <span>{timestamp.to_string()}</span>
                        </div>
                        <div>
                            {match &message.data {
                                MessageData::Party { .. } => {
                                    Some(
                                        view! {
                                            <svg viewBox="0 0 18 18">
                                                <path d="m9 4a1 1 90 0 0 0 5 1 1 90 0 0 0 -5m-4 13c0-5 1-7 4-7s4 2 4 7q-4 2-8 0m0-17a1 1 90 0 0 0 5 1 1 90 0 0 0 -5m-4 13c0-5 1-7 4-7 0.375 0 0.5 0 1.25 0.125-0.25 1.625 1.25 3.125 2.5 3.125q0.125 0.25 0.125 0.5c-1.75 0-3.625 1-3.875 4.125q-2.375 0-4-0.875m12-13a1 1 90 0 1 0 5 1 1 90 0 1 0 -5m4 13c0-5-1-7-4-7-0.375 0-0.5 0-1.25 0.125 0.25 1.625-1.25 3.125-2.5 3.125q-0.125 0.25-0.125 0.5c1.75 0 3.625 1 3.875 4.125q2.375 0 4-0.875" />
                                            </svg>
                                        }
                                            .into_any(),
                                    )
                                }
                                MessageData::Global { .. } => {
                                    Some(
                                        view! {
                                            <svg viewBox="0 0 18 18">
                                                <path d="m0.5 6h6l8.625-3.75q0.375 0 0.375 0.375v12.75q0 0.375-0.375 0.375l-8.625-3.75h-3c-3.75 0-3.75-6 0-6m12.375 1.5h0.375q0.75 0 0.75 0.75v1.5q0 0.75-0.75 0.75h-0.375v-3m-9.75 4.875-0.9375 2.8125q-0.1875 0.5625-0.5625 0.5625h-1.6875q-0.1875 0 0-0.5625l0.9375-2.8125h2.25" />
                                            </svg>
                                        }
                                            .into_any(),
                                    )
                                }
                                _ => None,
                            }}
                            <Author uuid=author.clone() /> <span>{text.to_string()}</span>
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
