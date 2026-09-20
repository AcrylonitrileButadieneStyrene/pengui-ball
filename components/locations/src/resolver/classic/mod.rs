use std::{
    collections::HashMap,
    sync::{Arc, nonpoison::Mutex},
};

use leptos::prelude::*;

mod coordinates;
use coordinates::Coordinates;

pub type Container = Mutex<HashMap<Arc<str>, LocalResource<Result<LocationData, gloo_net::Error>>>>;

#[derive(Debug, serde::Deserialize)]
pub struct LocationData {
    #[serde(rename = "ignoredMapIds")]
    pub ignored: Vec<Arc<str>>,
    #[serde(rename = "urlRoot", default)]
    pub root: Option<Arc<str>>,
    #[serde(rename = "mapLocations")]
    pub maps: HashMap<Arc<str>, LocationItem>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum LocationItem {
    Literal(Arc<str>),
    Object {
        title: Arc<str>,
        #[serde(rename = "urlTitle")]
        url_title: Option<Arc<str>>,
        coords: Option<Coordinates>,
        #[serde(default)]
        explorer: bool,
    },
    Array(Vec<Self>),
    Dynamic(HashMap<Arc<str>, Self>),
}

#[derive(Debug, serde::Deserialize)]
pub struct Location {
    pub name: Arc<str>,
    pub wiki: Option<Arc<str>>,
}

pub fn fetch_with_owner(
    game: &str,
    owner: &Owner,
) -> LocalResource<Result<LocationData, gloo_net::Error>> {
    owner.with(|| fetch(game))
}

pub fn fetch(game: &str) -> LocalResource<Result<LocationData, gloo_net::Error>> {
    let endpoint = format!("/{game}/_yno/locations/{game}/config.json");
    LocalResource::new(move || {
        let endpoint = endpoint.clone();
        async move {
            gloo_net::http::Request::get(&endpoint)
                .send()
                .await?
                .json()
                .await
        }
    })
}

pub fn resolve(
    item: &LocationItem,
    previous: Option<u16>,
    x: i16,
    y: i16,
) -> Vec<(Arc<str>, Option<Arc<str>>)> {
    let result = resolve_inner(item, previous, x, y);
    let largest = result
        .iter()
        .map(|(_name, _wiki, priority)| *priority)
        .max()
        .unwrap_or_default();
    result
        .into_iter()
        .filter(|(_name, _wiki, priority)| *priority >= largest)
        .map(|(name, wiki, _priority)| (name, wiki))
        .collect()
}

fn resolve_inner(
    item: &LocationItem,
    previous: Option<u16>,
    x: i16,
    y: i16,
) -> Vec<(Arc<str>, Option<Arc<str>>, u8)> {
    match item {
        LocationItem::Literal(name) => vec![(name.clone(), None, 0)],
        LocationItem::Object {
            title,
            url_title,
            coords,
            ..
        } => {
            if coords.as_ref().is_none_or(|coords| coords.contains(x, y)) {
                vec![(title.clone(), url_title.clone(), 1)]
            } else {
                Vec::new()
            }
        }
        LocationItem::Array(items) => items
            .iter()
            .flat_map(|item| resolve_inner(item, previous, x, y))
            .collect(),
        LocationItem::Dynamic(items) => items
            .iter()
            .flat_map(|(from, item)| {
                let from = &**from;

                if previous.map_or(from == "else", |prev| from == format!("{prev:>04}")) {
                    resolve_inner(item, previous, x, y)
                } else {
                    Vec::new()
                }
            })
            .collect(),
    }
}
