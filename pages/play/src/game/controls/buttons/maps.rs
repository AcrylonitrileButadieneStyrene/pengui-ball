use leptos::prelude::*;

use crate::{game::controls::icon, state::api::locations::ResolvedLocation};

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
    let location = state.game.location;
    let is_2kki = &*state.game.id.clone() == "2kki";

    let maps = LocalResource::<Vec<Map>>::new(move || {
        let state = state.clone();
        async move {
            let Some(location) = location.get() else {
                return vec![];
            };

            let endpoint = match (is_2kki, state.api.locations.resolve(&location)) {
                (_, ResolvedLocation::Pending | ResolvedLocation::Unknown) => return vec![],
                (_, ResolvedLocation::Multiple(locations)) if locations.is_empty() => return vec![],
                (true, ResolvedLocation::Single { name, .. }) => [EXPLORER_BASE, &name].concat(),
                (true, ResolvedLocation::Multiple(locations)) => [
                    EXPLORER_BASE,
                    &locations
                        .iter()
                        .map(|location| location.title.clone())
                        .collect::<Vec<_>>()
                        .join("&locationNames="),
                ]
                .concat(),
                (false, ResolvedLocation::Single { name, .. }) => [WIKI_BASE, &name].concat(),
                (false, ResolvedLocation::Multiple(locations)) => [
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
                        <a role="button" href=wiki_link title=description target="yumeWikiMap">
                            <icon::Map />
                        </a>
                    }
                })
                .collect::<Vec<_>>()
        })
    }
}
