use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

stylance::import_style!(pub style, "status.module.css");

#[island]
pub fn Status() -> impl IntoView {
    let ready_state = use_context::<Signal<ConnectionReadyState>>().unwrap();
    let connected = move || matches!(ready_state.get(), ConnectionReadyState::Open);
    let connecting = move || matches!(ready_state.get(), ConnectionReadyState::Connecting);

    view! {
        <span class=style::indicator class:connected=connected class:connecting=connecting>
            {"\u{25cf}"}
        </span>
        {move || {
            if connected() {
                "Connected"
            } else if connecting() {
                "Connecting"
            } else {
                "Disconnected"
            }
        }}
    }
}
