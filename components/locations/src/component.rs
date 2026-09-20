use std::sync::Arc;

use leptos::prelude::*;

use crate::{LocationResolved, resolver::classic};

#[component]
#[allow(non_snake_case)]
pub fn Location(
    #[prop(into, optional)] location: Option<Signal<Option<crate::Location>>>,
    #[prop(into, optional)] resolved: Option<Signal<Option<crate::LocationResolved>>>,
) -> impl IntoView {
    location.map_or_else(
        || {
            resolved.map_or_else(
                || panic!("Location component requires either `location` or `resolved` prop"),
                |resolved| {
                    let view = move || resolved.get().and_then(location_resolved_inner);
                    view! { <Suspense fallback=|| ()>{view}</Suspense> }.into_any()
                },
            )
        },
        |location| {
            let view = move || location.get().and_then(location_inner);
            view! { <Suspense fallback=|| ()>{view}</Suspense> }.into_any()
        },
    )
}

fn location_inner(location: crate::Location) -> Option<impl IntoView> {
    location_resolved_inner(expect_context::<Arc<crate::Resolver>>().resolve(&location))
}

fn location_resolved_inner(location: crate::LocationResolved) -> Option<AnyView> {
    let result = match location {
        LocationResolved::None | LocationResolved::Pending => return None,
        LocationResolved::Unknown { map, x, y } => {
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
                            href=format!("https://yume.wiki/2kki/{}", world.title)
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
