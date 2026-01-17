use leptos::prelude::*;

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(State::new());
    children()
}

pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}
