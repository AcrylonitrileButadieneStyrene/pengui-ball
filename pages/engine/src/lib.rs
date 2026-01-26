#![allow(non_snake_case)]

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod callbacks;
mod easyrpg;
mod effects;
mod state;

pub use easyrpg::messages::send;

pub type EngineState = std::sync::Arc<state::EngineState>;

stylance::import_style!(pub style, "lib.module.css");

#[component]
pub fn Engine() -> impl IntoView {
    let game = use_params_map().get().get("game").unwrap();

    view! {
        <leptos_meta::Body {..} class=style::engine />
        <leptos_meta::Link rel="stylesheet" href="/css/engine.css" />

        <state::Provider game=game.clone()>
            <easyrpg::EasyRPG game>{None::<()>}</easyrpg::EasyRPG>
            <effects::Effects />
        </state::Provider>
    }
}
