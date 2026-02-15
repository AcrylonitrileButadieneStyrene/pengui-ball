use std::sync::Arc;

use leptos::prelude::*;

use crate::state::{api::location::LocationItem, game::Location};

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
        map,
        previous,
        x,
        y,
    } = location;

    let state = crate::state();
    let locations = state.api.locations.get(&state.game.id);

    if let Some(Ok(locations)) = &*locations.read()
        && let Some(map) = locations.maps.get(&*format!("{map:>04}"))
        && let Some((name, url)) = find_map(map, previous, x, y)
    {
        let url = locations.root.to_string() + &url.unwrap_or_else(|| name.clone());

        view! {
            <a href=url target="yumeWiki">
                {name}
            </a>
        }
        .into_any()
    } else {
        view! {
            <span>Unknown Location: {format!("Map{map:>04}({x}, {y})")}</span>
        }
        .into_any()
    }
}

fn find_map(
    map: &LocationItem,
    previous: Option<u16>,
    x: u16,
    y: u16,
) -> Option<(Arc<str>, Option<Arc<str>>)> {
    match map {
        LocationItem::Literal(name) => Some((name.clone(), None)),
        LocationItem::Object {
            title,
            url_title,
            coords,
            ..
        } => {
            if coords.as_ref().is_some_and(|coords| !coords.contains(x, y)) {
                None
            } else {
                Some((title.clone(), url_title.clone()))
            }
        }
        LocationItem::Array(items) => items.iter().find_map(|item| find_map(item, previous, x, y)),
        LocationItem::Dynamic(items) => items.iter().find_map(|(from, item)| {
            let from = &**from;
            let matching = if let Some(prev) = previous
                && from == format!("{prev:>04}")
            {
                true
            } else {
                from == "else"
            };

            if matching {
                find_map(item, previous, x, y)
            } else {
                None
            }
        }),
    }
}
