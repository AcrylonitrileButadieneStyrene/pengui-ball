use std::sync::Arc;

use leptos::prelude::*;

use crate::{LocationResolved, resolver::classic};

#[component]
#[allow(non_snake_case)]
pub fn Location(#[prop(into)] location: Signal<Option<crate::Location>>) -> impl IntoView {
    let view = move || location.get().and_then(location_inner);
    view! { <Suspense fallback=|| ()>{view}</Suspense> }
}

fn location_inner(location: crate::Location) -> Option<impl IntoView> {
    let result = match expect_context::<Arc<crate::Resolver>>().resolve(&location) {
        LocationResolved::Pending => return None,
        LocationResolved::Unknown => {
            let crate::Location { map, x, y, .. } = location;
            view! { <span>{format!("Map{map:>04}({x}, {y})")}</span> }.into_any()
        }
        LocationResolved::Classic(worlds) => wrap_multiple(
            worlds
                .iter()
                .map(classic_world_inner)
                .intersperse_with(|| view! { <span>{" | "}</span> }.into_any())
                .collect::<Vec<_>>(),
        ),
        LocationResolved::Explorer(worlds) => wrap_multiple(
            worlds
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
                .intersperse_with(|| view! { <span>{" | "}</span> }.into_any())
                .collect::<Vec<_>>(),
        ),
    };

    Some(result)
}

fn wrap_multiple(views: Vec<AnyView>) -> AnyView {
    if views.len() > 1 {
        view! { <span>{views}</span> }.into_any()
    } else {
        views.into_any()
    }
}

fn classic_world_inner(world: &classic::Location) -> AnyView {
    let name = world.name.clone();
    match &world.wiki {
        None => view! { <span>{name}</span> }.into_any(),
        Some(wiki) => view! { <a href=wiki.clone() target="yumeWiki">{name}</a> }.into_any(),
    }
}
