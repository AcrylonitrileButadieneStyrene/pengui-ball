#![allow(non_snake_case)]

use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod callbacks;
mod easyrpg;
mod effects;
mod state;

pub use easyrpg::messages::send;

pub type EngineState = std::sync::Arc<state::EngineState>;

#[component]
pub fn Engine() -> impl IntoView {
    let game = Arc::<str>::from(use_params_map().get().get("game").unwrap());

    view! {
        <leptos_meta::Body {..} style:overflow="hidden" />
        <leptos_meta::Meta name="robots" content="noindex" />
        <leptos_meta::Style>
            r"#canvas {
                width: 100vw;
                height: 100vh;
                image-rendering: pixelated;
                image-rendering: crisp-edges;
            }"#
        </leptos_meta::Style>

        <state::Provider game=game.clone()>
            <easyrpg::EasyRPG game>{None::<()>}</easyrpg::EasyRPG>
            <effects::Effects />
        </state::Provider>
    }
}
