use leptos::prelude::*;

#[island]
pub fn CurrentLocation() -> impl IntoView {
    let state = crate::state();
    view! { <locations::Location resolved=state.locations.current_resolved /> }
}
