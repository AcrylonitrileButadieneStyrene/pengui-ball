use std::sync::Arc;

use leptos::prelude::*;

pub mod locations;

pub struct State {
    pub user: LocalResource<Option<super::user::User>>,
    pub locations: locations::Locations,
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
            locations: locations::Locations::new_prefetch(game),
        }
    }
}
