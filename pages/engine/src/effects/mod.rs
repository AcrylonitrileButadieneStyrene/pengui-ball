use leptos::prelude::*;

pub mod events;
mod save_timestamps;
mod volume;

#[island]
pub fn Effects() -> impl IntoView {
    let state = expect_context::<crate::EngineState>();

    volume::effect(state.clone());
    events::focus::effect(state.clone());
    events::error::effect();
    save_timestamps::effect(state);
}
