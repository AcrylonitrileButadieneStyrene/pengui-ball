use std::sync::Arc;

use leptos::prelude::*;

use crate::LocationResolved;

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
        LocationResolved::Classic {
            name,
            wiki: Some(wiki),
        } => view! {
            <a href=wiki target="yumeWiki">
                {name}
            </a>
        }
        .into_any(),
        LocationResolved::Classic { name, wiki: None } => view! { <span>{name}</span> }.into_any(),
        LocationResolved::Explorer(worlds) => {
            let nodes = worlds
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
                .collect::<Vec<_>>();
            if nodes.len() > 1 {
                view! { <span>{nodes}</span> }.into_any()
            } else {
                nodes.into_any()
            }
        }
    };

    Some(result)
}
