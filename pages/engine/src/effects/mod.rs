use leptos::prelude::*;

mod volume;

#[island]
pub fn Effects() -> impl IntoView {
    let state = expect_context::<std::sync::Arc<crate::EngineState>>();

    volume::effect(state);
}
