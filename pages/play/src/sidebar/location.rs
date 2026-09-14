use leptos::prelude::*;

use locations::Location;

#[island]
pub fn CurrentLocation() -> impl IntoView {
    let state = crate::state();
    view! { <Location location=state.locations.current /> }
}
