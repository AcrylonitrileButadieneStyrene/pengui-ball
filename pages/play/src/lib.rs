#![feature(result_option_map_or_default)]
#![allow(non_snake_case)]

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod game;
mod header;
mod messages;
mod sidebar;
mod state;

pub use state::State;

stylance::import_style!(pub style, "lib.module.css");

pub type CurrentGame = std::sync::Arc<common::config::Game>;

#[component]
pub fn Play() -> impl IntoView {
    let id = use_params_map().get().get("game").unwrap();
    let config = expect_context::<std::sync::Arc<common::Config>>();
    let games = config.games.clone();

    let Some(game) = games.into_iter().find(|game| game.id == id) else {
        return leptos::either::Either::Left(
            view! { <leptos_router::components::Redirect path="/" /> },
        );
    };

    let game = std::sync::Arc::new(game);
    provide_context(game.clone());

    leptos::either::Either::Right(view! {
        <leptos_meta::Link rel="stylesheet" href="/pkg/play.css" />
        <leptos_meta::Title text=format!("{} Online - YNOproject", game.name) />
        <leptos_meta::Meta
            name="description"
            content=format!(
                "Play multiplayer {} for free! Ad-free and no registration required.",
                game.name,
            )
        />
        <leptos_meta::Body {..} class=style::game />

        <state::Provider>
            <main class=style::layout>
                <header::Header />
                <game::Game />
                <sidebar::Sidebar />
            </main>
            <messages::Handler />
        </state::Provider>
    })
}
