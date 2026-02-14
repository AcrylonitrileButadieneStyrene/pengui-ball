use std::sync::Arc;

use leptos::prelude::*;

pub mod location;

pub struct State {
    pub user: LocalResource<Option<super::user::User>>,
    pub locations: location::Locations,
}

impl State {
    pub fn new(game: Arc<str>) -> Self {
        Self {
            user: LocalResource::new(|| async {
                gloo_net::http::Request::get("api/info")
                    .send()
                    .await
                    .ok()?
                    .json()
                    .await
                    .ok()?
            }),
            locations: location::Locations::new_prefetch(game),
        }
    }
}
