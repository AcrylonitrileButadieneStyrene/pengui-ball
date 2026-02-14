use std::{
    collections::HashMap,
    sync::{Arc, nonpoison::RwLock},
};

use leptos::prelude::*;

#[derive(Debug, serde::Deserialize)]
pub struct LocationData {
    #[serde(rename = "ignoredMapIds")]
    pub ignored: Vec<Arc<str>>,
    #[serde(rename = "urlRoot")]
    pub root: Arc<str>,
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
    },
    Array(Vec<LocationItem>),
}

#[derive(Debug, serde::Deserialize)]
pub struct Coordinates {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

impl Coordinates {
    pub fn contains(&self, x: u16, y: u16) -> bool {
        (if self.x1 == -1 {
            true
        } else {
            self.x1 as u16 <= x
        }) && if self.x2 == -1 {
            true
        } else {
            x <= self.x2 as u16
        } && if self.y1 == -1 {
            true
        } else {
            self.y1 as u16 <= y
        } && if self.y2 == -1 {
            true
        } else {
            y <= self.y2 as u16
        }
    }
}

// maybe replace this with lazy_map
pub struct Locations(
    RwLock<HashMap<Arc<str>, LocalResource<Result<LocationData, gloo_net::Error>>>>,
);

impl Locations {
    pub fn new_prefetch(game: Arc<str>) -> Self {
        let mut map = HashMap::new();
        let resource = resource(&game);
        map.insert(game, resource);
        Self(RwLock::new(map))
    }

    pub fn get(&self, game: &str) -> LocalResource<Result<LocationData, gloo_net::Error>> {
        if let Some(resource) = self.0.read().get(game) {
            *resource
        } else {
            let resource = resource(game);
            self.0.write().insert(Arc::from(game), resource);
            resource
        }
    }
}

fn resource(game: &str) -> LocalResource<Result<LocationData, gloo_net::Error>> {
    let endpoint = format!("/yno/{game}/locations/{game}/config.json");
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
