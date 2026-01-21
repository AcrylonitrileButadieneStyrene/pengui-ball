#![allow(non_snake_case)]

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod callbacks;
mod easyrpg;
mod effects;
mod messages;
mod state;

pub use state::EngineState;

stylance::import_style!(pub style, "lib.module.css");

#[component]
pub fn Engine() -> impl IntoView {
    let game = use_params_map().get().get("game").unwrap();

    view! {
        <leptos_meta::Body {..} class=style::engine />
        <leptos_meta::Link rel="stylesheet" href="/pkg/engine.css" />

        <state::Provider game=game.clone()>
            <easyrpg::EasyRPG game>{None::<()>}</easyrpg::EasyRPG>
            <messages::Handler />
            <effects::Effects />
        </state::Provider>
    }
}
