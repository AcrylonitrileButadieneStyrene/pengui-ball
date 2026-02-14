#![feature(result_option_map_or_default)]
#![feature(nonpoison_rwlock)]
#![feature(sync_nonpoison)]
#![allow(non_snake_case)]

use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod game;
mod header;
mod modals;
mod sidebar;
mod state;

stylance::import_style!(pub style, "lib.module.css");

pub type CurrentGame = Arc<common::config::Game>;
pub type State = Arc<state::PlayState>;

pub fn state() -> State {
    expect_context::<State>()
}

#[component]
pub fn Play() -> impl IntoView {
    let id = use_params_map().get().get("game").unwrap();
    let config = expect_context::<Arc<common::ServerConfiguration>>();
    let games = config.games.clone();

    let Some(game) = games.into_iter().find(|game| &*game.id == &id) else {
        return view! { <leptos_router::components::Redirect path="/" /> }.into_any();
    };

    let game = Arc::new(game);
    provide_context(game.clone());

    view! {
        <leptos_meta::Link rel="stylesheet" href="/css/play.css" />
        <leptos_meta::Link rel="stylesheet" href="/css/themes.css" />
        <leptos_meta::Title text=format!("{} Online - YNOproject", game.name) />
        <leptos_meta::Meta
            name="description"
            content=format!(
                "Play multiplayer {} for free! Ad-free and no registration required.",
                game.name,
            )
        />
        <leptos_meta::Body {..} class=style::game />

        <state::Provider game_id=game.id.clone()>
            <main id="layout" class=style::layout>
                <header::Header />
                <game::Game />
                <sidebar::Sidebar />
            </main>
            <modals::Modals />
        </state::Provider>

        <PermissionDisclaimer permission=game.permission />
    }
    .into_any()
}

#[component]
fn PermissionDisclaimer(permission: common::config::PermissionStatus) -> impl IntoView {
    use common::config::PermissionStatus;
    match permission {
        PermissionStatus::Yume1kki => "Pending approval from developer/publisher",
        PermissionStatus::Yume2kki => "Hosted with permission from the Yume 2kki developers",
        // PermissionStatus::CU => "Original disappointment by the YNOproject community",
        PermissionStatus::Pending => "Hosted with permission from the developer(s)",
    }
}
