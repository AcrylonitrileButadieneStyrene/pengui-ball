use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

mod callbacks;
mod easyrpg;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Player() -> impl IntoView {
    let game = use_query_map().get().get("game").unwrap();

    view! {
        <leptos_meta::Body {..} class=style::player />
        <easyrpg::EasyRPG game>
            <div></div>
        </easyrpg::EasyRPG>
    }
}

// #[island]
// fn Setup() -> impl IntoView {
//     let handle = window_event_listener(leptos::ev::message, |msg| {
//         leptos::logging::log!("message {msg:?}")
//     });
//     on_cleanup(|| handle.remove());
// }
