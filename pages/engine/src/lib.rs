#![allow(non_snake_case)]

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

mod callbacks;
mod easyrpg;
mod state;

pub use state::EngineState;

stylance::import_style!(pub style, "lib.module.css");

#[component]
pub fn Engine() -> impl IntoView {
    let game = use_query_map().get().get("game").unwrap();

    view! {
        <leptos_meta::Body {..} class=style::engine />
        <leptos_meta::Link rel="stylesheet" href="pkg/engine.css" />

        <state::Provider>
            <easyrpg::EasyRPG game>{None::<()>}</easyrpg::EasyRPG>
        </state::Provider>
    }
}

// #[island]
// fn Setup() -> impl IntoView {
//     let handle = window_event_listener(leptos::ev::message, |msg| {
//         leptos::logging::log!("message {msg:?}")
//     });
//     on_cleanup(|| handle.remove());
// }
