use std::sync::Arc;

use leptos::prelude::*;

use crate::states::players::player::PlayerStoreFields as _;

stylance::import_style!(pub style, "author.module.css");

#[component]
pub fn Author(uuid: Arc<str>) -> impl IntoView {
    let state = crate::state();
    let author = state
        .players
        .all
        .with_untracked(|players| players.get(&uuid).copied());

    author
        .map(|player| {
            move || {
                let (name_start, name_end) = if *player.account().read() {
                    ("[", "]")
                } else {
                    ("<", ">")
                };

                let badge = player.badge().read().as_ref().map(|badge| {
                    view! {
                        <img
                            class=style::badge
                            src=format!("https://ynoproject.net/2kki/images/badge/{badge}.png")
                        />
                    }
                });

                let (gradient, shadow) = player
                    .system()
                    .read()
                    .as_ref()
                    .map(|sys| {
                        (
                            format!("var(--{}-{sys}-gradient)", state.locations.game),
                            format!("var(--{}-{sys}-shadow)", state.locations.game),
                        )
                    })
                    .unzip();

                view! {
                    <div class=style::author>
                        {name_start}
                        <span
                            class=style::name
                            style:background-image=gradient
                            style=("--shadow-color", shadow)
                        >
                            {player.name().get()}
                        </span> {badge} {name_end}
                    </div>
                }
            }
        })
        .into_any()
}
