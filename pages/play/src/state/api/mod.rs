use std::sync::Arc;

use leptos::prelude::*;

pub mod locations;
pub mod user;

pub struct State {
    pub user: LocalResource<Result<user::User, user::UserError>>,
    pub locations: locations::Locations,
}

impl State {
    pub fn new(game: Arc<str>) -> Self {
        Self {
            user: user::resource(),
            locations: locations::Locations::new_prefetch(game),
        }
    }
}
