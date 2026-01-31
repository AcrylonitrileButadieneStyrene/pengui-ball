use leptos::prelude::*;

mod events;
mod volume;

#[island]
pub fn Effects() -> impl IntoView {
    let state = expect_context::<crate::EngineState>();

    volume::effect(state.clone());
    events::effect(state);
}
