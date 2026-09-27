use leptos::prelude::*;
use locations::LocationResolved;

const EXPLORER_BASE: &str = "/explorer/getLocationMaps?locationNames=";
const WIKI_BASE: &str = "https://wrapper.yume.wiki/maps?game=2kki&location=";

#[derive(Debug, Clone)]
pub enum Map {
    WikiMap(WikiMap),
    ExplorerMap(ExplorerMap),
}

impl Map {
    pub fn extract(self) -> (String, String) {
        let (Map::WikiMap(WikiMap {
            path: wiki_link,
            caption: description,
        })
        | Map::ExplorerMap(ExplorerMap {
            url: wiki_link,
            label: description,
        })) = self;

        (wiki_link, description)
    }

    pub fn extract_ref(&self) -> (&str, &str) {
        let (Map::WikiMap(WikiMap {
            path: wiki_link,
            caption: description,
        })
        | Map::ExplorerMap(ExplorerMap {
            url: wiki_link,
            label: description,
        })) = self;

        (wiki_link, description)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WikiMap {
    pub path: String,
    pub caption: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ExplorerMap {
    pub url: String,
    pub label: String,
}

pub fn resource(
    current_resolved: Signal<Option<locations::LocationResolved>>,
) -> LocalResource<Vec<Map>> {
    LocalResource::<Vec<Map>>::new(move || async move {
        let Some(location) = current_resolved.get() else {
            return vec![];
        };

        let (locations, is_explorer) = match location {
            LocationResolved::None
            | LocationResolved::Pending
            | LocationResolved::Unknown { .. } => return vec![],
            LocationResolved::Explorer(locations) if locations.is_empty() => return vec![],
            LocationResolved::Classic(locations) if locations.is_empty() => return vec![],
            LocationResolved::Classic(locations) => (
                &locations
                    .iter()
                    .map(|location| location.name.clone())
                    .collect::<Vec<_>>(),
                false,
            ),
            LocationResolved::Explorer(locations) => (
                &locations
                    .iter()
                    .map(|location| location.title.clone())
                    .collect::<Vec<_>>(),
                true,
            ),
        };

        let base = if is_explorer {
            EXPLORER_BASE
        } else {
            WIKI_BASE
        };

        let endpoint = [base, &locations.join("&locationNames=")].concat();
        let Ok(request) = gloo_net::http::Request::get(&endpoint).send().await else {
            return vec![];
        };

        if is_explorer {
            request
                .json::<Vec<_>>()
                .await
                .unwrap_or_default()
                .into_iter()
                .map(Map::ExplorerMap)
                .collect()
        } else {
            request
                .json::<Vec<_>>()
                .await
                .unwrap_or_default()
                .into_iter()
                .map(Map::WikiMap)
                .collect()
        }
    })
}
