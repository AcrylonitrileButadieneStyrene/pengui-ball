use std::sync::Arc;

use leptos::prelude::*;

#[derive(serde::Deserialize)]
pub struct Screenshot {
    pub id: Arc<str>,
    pub uuid: Arc<str>,
    pub game: Arc<str>,
    #[serde(rename = "mapId")]
    pub map: Arc<str>,
    #[serde(rename = "mapX")]
    pub map_x: i16,
    #[serde(rename = "mapY")]
    pub map_y: i16,
    #[serde(rename = "systemName")]
    pub system: Arc<str>,
    pub timestamp: Arc<str>,
    pub public: bool,
    pub spoiler: bool,
    #[serde(rename = "likeCount")]
    pub likes: u16,
    pub liked: bool,
}

pub fn resource(game: &str) -> LocalResource<Vec<Screenshot>> {
    let endpoint: std::sync::Arc<str> =
        format!("https://api.ynoproject.net/{game}/api/screenshot?command=getPlayerScreenshots")
            .into();
    LocalResource::new(move || {
        let endpoint = endpoint.clone();
        async move {
            let Ok(response) = gloo_net::http::Request::get(&endpoint).send().await else {
                return vec![];
            };

            response.json().await.unwrap_or_default()
        }
    })
}
