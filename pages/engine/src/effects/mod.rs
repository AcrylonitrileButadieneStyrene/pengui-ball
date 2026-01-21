use leptos::prelude::*;

mod volume;

#[island]
pub fn Effects() -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::EngineState>>().unwrap();

    volume::effect(state);
}
