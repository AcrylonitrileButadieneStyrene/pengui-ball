use leptos::prelude::*;

mod callbacks;

#[component]
pub fn Player() -> impl IntoView {
    view! {
        <canvas id="canvas" width=320 height=240 />
        <script src="data/ynoengine-simd.js" />
        <script>createEasyRpgPlayer()</script>
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
