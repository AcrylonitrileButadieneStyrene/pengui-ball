use std::sync::Arc;

use leptos::prelude::*;

use crate::states::players::player::PlayerStoreFields as _;

stylance::import_style!(pub style, "author.module.css");

#[component]
pub fn Author(uuid: Arc<str>, icon: AnyView) -> impl IntoView {
    let state = crate::state();
    let author = state
        .players
        .all
        .with_untracked(|players| players.get(&uuid).copied())
        .unwrap();

    let wrapper = move || {
        if *author.account().read() {
            ("[", "]")
        } else {
            ("<", ">")
        }
    };

    let badge = move || {
        author.badge().read().as_ref().map(|badge| {
            view! {
                <img
                    class=style::badge
                    src=format!("https://ynoproject.net/2kki/images/badge/{badge}.png")
                />
            }
        })
    };

    let name = move || {
        author.system().read().as_ref().map_or_else(
            || author.name().get().into_any(),
            |system| {
                author_name_with_system(author.name().get(), system, &state.locations.game)
                    .into_any()
            },
        )
    };

    view! {
        <div class=style::author>
            {icon} {move || wrapper().0} {name} {badge} {move || wrapper().1}
        </div>
    }
}

fn author_name_with_system(name: Option<Arc<str>>, system: &str, game: &str) -> impl IntoView {
    view! {
        <span
            class=style::name
            style:background-image=format!("var(--{}-{system}-gradient)", game)
            style=("--shadow-color", format!("var(--{}-{system}-shadow)", game))
        >
            {name}
        </span>
    }
}
