use leptos::prelude::*;

use crate::{game::controls::icon, states::locations::LocationResolved};

#[derive(Debug, Clone)]
enum Map {
    WikiMap(WikiMap),
    ExplorerMap(ExplorerMap),
}

#[derive(Debug, Clone, serde::Deserialize)]
struct WikiMap {
    path: String,
    caption: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ExplorerMap {
    url: String,
    label: String,
}

const EXPLORER_BASE: &str = "/explorer/getLocationMaps?locationNames=";
const WIKI_BASE: &str = "https://wrapper.yume.wiki/maps?game=2kki&location=";

#[island]
pub fn Maps() -> impl IntoView {
    let state = crate::state();
    let is_2kki = &*state.locations.game.clone() == "2kki";

    let maps = LocalResource::<Vec<Map>>::new(move || {
        let state = state.clone();
        async move {
            let Some(location) = state.locations.current_resolved.get() else {
                return vec![];
            };

            let endpoint = match (is_2kki, location) {
                (_, LocationResolved::Pending | LocationResolved::Unknown) => return vec![],
                (_, LocationResolved::Multiple(locations)) if locations.is_empty() => return vec![],
                (true, LocationResolved::Single { name, .. }) => [EXPLORER_BASE, &name].concat(),
                (true, LocationResolved::Multiple(locations)) => [
                    EXPLORER_BASE,
                    &locations
                        .iter()
                        .map(|location| location.title.clone())
                        .collect::<Vec<_>>()
                        .join("&locationNames="),
                ]
                .concat(),
                (false, LocationResolved::Single { name, .. }) => [WIKI_BASE, &name].concat(),
                (false, LocationResolved::Multiple(locations)) => [
                    WIKI_BASE,
                    &locations
                        .first()
                        .map(|location| location.title.clone())
                        .unwrap_or_default(),
                ]
                .concat(),
            };

            let Ok(request) = gloo_net::http::Request::get(&endpoint).send().await else {
                return vec![];
            };

            if is_2kki {
                request
                    .json::<Vec<_>>()
                    .await
                    .unwrap()
                    .into_iter()
                    .map(Map::ExplorerMap)
                    .collect()
            } else {
                request
                    .json::<Vec<_>>()
                    .await
                    .unwrap()
                    .into_iter()
                    .map(Map::WikiMap)
                    .collect()
            }
        }
    });

    move || {
        maps.read().as_ref().map(|maps| {
            maps.clone()
                .into_iter()
                .map(|map| {
                    let (Map::WikiMap(WikiMap {
                        path: wiki_link,
                        caption: description,
                    })
                    | Map::ExplorerMap(ExplorerMap {
                        url: wiki_link,
                        label: description,
                    })) = map;

                    view! {
                        <a class="pop-out" href=wiki_link title=description target="yumeWikiMap">
                            <icon::Map />
                        </a>
                    }
                })
                .collect::<Vec<_>>()
        })
    }
}
