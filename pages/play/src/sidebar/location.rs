use std::sync::Arc;

use leptos::prelude::*;

use crate::state::api::location::LocationItem;

#[island]
pub fn CurrentLocation() -> impl IntoView {
    let state = crate::state();
    view! { <Location location=state.location /> }
}

#[component]
pub fn Location(#[prop(into)] location: Signal<Option<(u16, u16, u16)>>) -> impl IntoView {
    move || {
        location
            .get()
            .map(|(location, x, y)| location_inner(location, x, y))
    }
}

fn location_inner(location: u16, x: u16, y: u16) -> impl IntoView {
    let state = crate::state();
    let locations = state.api.locations.get(&state.game_id);

    if let Some(Ok(locations)) = &*locations.read()
        && let Some(map) = locations.maps.get(&*format!("{location:>04}"))
        && let Some((name, url)) = find_map(map, x, y)
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
            <span>Unknown Location: {format!("Map{location:>04}({x}, {y})")}</span>
        }
        .into_any()
    }
}

fn find_map(map: &LocationItem, x: u16, y: u16) -> Option<(Arc<str>, Option<Arc<str>>)> {
    match map {
        LocationItem::Literal(name) => Some((name.clone(), None)),
        LocationItem::Object {
            title,
            url_title,
            coords,
        } => {
            if coords.as_ref().is_some_and(|coords| !coords.contains(x, y)) {
                None
            } else {
                Some((title.clone(), url_title.clone()))
            }
        }
        LocationItem::Array(items) => items.iter().find_map(|item| find_map(item, x, y)),
    }
}
