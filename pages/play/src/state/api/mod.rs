use leptos::prelude::*;

pub mod user;

pub struct State {
    pub user: LocalResource<Result<user::User, user::UserError>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            user: user::resource(),
        }
    }
}
