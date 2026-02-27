use leptos::prelude::*;

pub mod user;

pub struct State {
    pub user: LocalResource<Result<user::User, user::UserError>>,
    pub has_account: Signal<bool>,
}

impl State {
    pub fn new() -> Self {
        let user = user::resource();

        Self {
            user,
            has_account: Signal::derive(move || {
                user.read()
                    .as_ref()
                    .is_some_and(|user| user.as_ref().ok().is_some_and(|user| user.registered))
            }),
        }
    }
}
