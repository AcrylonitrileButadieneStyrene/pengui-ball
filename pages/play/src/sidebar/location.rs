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
    let Location { map, x, y, .. } = location;
    let state = crate::state();
    match state.api.locations.resolve(&state.game.id, location) {
        Some(ResolvedLocation {
            name,
            wiki: Some(wiki),
        }) => view! {
            <a href=wiki target="yumeWiki">
                {name}
            </a>
        }
        .into_any(),
        Some(ResolvedLocation { name, wiki: None }) => view! { <span>{name}</span> }.into_any(),
        None => {
            view! { <span>Unknown Location: {format!("Map{map:>04}({x}, {y})")}</span> }.into_any()
        }
    }
}
