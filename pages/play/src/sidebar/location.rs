use leptos::prelude::*;

use crate::state::{api::locations::ResolvedLocation, game::Location};

#[island]
pub fn CurrentLocation() -> impl IntoView {
    let state = crate::state();
    view! { <Location location=state.game.location /> }
}

#[component]
pub fn Location(#[prop(into)] location: Signal<Option<Location>>) -> impl IntoView {
    move || location.get().map(location_inner)
}

fn location_inner(location: Location) -> impl IntoView {
    let Location {
        ref game,
        map,
        x,
        y,
        ..
    } = location;
    match crate::state().api.locations.resolve(&location) {
        Some(ResolvedLocation::Single {
            name,
            wiki: Some(wiki),
        }) => view! {
            <a href=wiki target="yumeWiki">
                {name}
            </a>
        }
        .into_any(),
        Some(ResolvedLocation::Single { name, wiki: None }) => {
            view! { <span>{name}</span> }.into_any()
        }
        Some(ResolvedLocation::Multiple(worlds)) => worlds
            .iter()
            .map(|world| {
                view! {
                    <a href=format!("https://yume.wiki/{game}/{}", world.title) target="yumeWiki">
                        {world.title.clone()}
                    </a>
                }
                .into_any()
            })
            .intersperse_with(|| "|".into_any())
            .collect::<Vec<_>>()
            .into_any(),
        None => {
            view! { <span>Unknown Location: {format!("Map{map:>04}({x}, {y})")}</span> }.into_any()
        }
    }
}
