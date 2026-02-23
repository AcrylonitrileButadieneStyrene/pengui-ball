use leptos::prelude::*;

use crate::state::{api::locations::ResolvedLocation, game::Location};

#[island]
pub fn CurrentLocation() -> impl IntoView {
    let state = crate::state();
    view! { <Location location=state.game.location /> }
}

#[component]
pub fn Location(#[prop(into)] location: Signal<Option<Location>>) -> impl IntoView {
    let view = move || location.get().and_then(location_inner);
    view! { <Suspense fallback=|| ()>{view}</Suspense> }
}

fn location_inner(location: Location) -> Option<impl IntoView> {
    let view = match crate::state().api.locations.resolve(&location) {
        ResolvedLocation::Pending => return None,
        ResolvedLocation::Unknown => {
            let Location { map, x, y, .. } = location;
            view! { <span>Unknown Location: {format!("Map{map:>04}({x}, {y})")}</span> }.into_any()
        }
        ResolvedLocation::Single {
            name,
            wiki: Some(wiki),
        } => view! {
            <a href=wiki target="yumeWiki">
                {name}
            </a>
        }
        .into_any(),
        ResolvedLocation::Single { name, wiki: None } => {
            view! { <span>{name}</span> }.into_any()
        }
        ResolvedLocation::Multiple(worlds) => worlds
            .iter()
            .map(|world| {
                view! {
                    <a
                        href=format!("https://yume.wiki/{}/{}", location.game, world.title)
                        target="yumeWiki"
                    >
                        {world.title.clone()}
                    </a>
                }
                .into_any()
            })
            .intersperse_with(|| "|".into_any())
            .collect::<Vec<_>>()
            .into_any(),
    };

    Some(view)
}
