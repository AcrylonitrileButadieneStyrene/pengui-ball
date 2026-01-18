use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

stylance::import_style!(pub style, "status.module.css");

#[island]
pub fn Status() -> impl IntoView {
    let ready_state = use_context::<Signal<ConnectionReadyState>>().unwrap();

    view! {
        <div
            class=style::indicator
            class:connected=move || matches!(ready_state.get(), ConnectionReadyState::Open)
            class:connecting=move || matches!(ready_state.get(), ConnectionReadyState::Connecting)
        >
            {"\u{25cf}"}
        </div>
        <div>
            {move || match ready_state.get() {
                ConnectionReadyState::Open => "Connected",
                ConnectionReadyState::Connecting => "Connecting",
                ConnectionReadyState::Closing | ConnectionReadyState::Closed => "Disconnected",
            }}
        </div>
    }
}
