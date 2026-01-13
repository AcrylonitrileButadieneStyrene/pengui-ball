use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

mod callbacks;

#[component]
pub fn Player() -> impl IntoView {
    let game = use_query_map().get().get("game").unwrap();

    view! {
        <canvas id="canvas" />
        <script src=format!("yno/{game}/ynoengine-simd.js") />
        <script>player = createEasyRpgPlayer()</script>
        <Setup />
    }
}

#[island]
fn Setup() -> impl IntoView {
    Effect::new(callbacks::setup);

    let handle = window_event_listener(leptos::ev::message, |msg| {
        leptos::logging::log!("message {msg:?}")
    });
    on_cleanup(|| handle.remove());
}
