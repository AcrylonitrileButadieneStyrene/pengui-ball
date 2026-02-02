use std::sync::Arc;

use leptos::prelude::*;

stylance::import_style!(pub style, "author.module.css");

#[island]
pub fn Author(uuid: Arc<str>) -> impl IntoView {
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
