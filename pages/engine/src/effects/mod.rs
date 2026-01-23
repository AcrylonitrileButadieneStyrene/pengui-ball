use leptos::prelude::*;

mod volume;

#[island]
pub fn Effects() -> impl IntoView {
    let state = expect_context::<crate::EngineState>();

    volume::effect(state);
}
