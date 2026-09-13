#![feature(nonpoison_mutex)]
#![feature(sync_nonpoison)]
#![feature(iter_intersperse)]
#![feature(random)]
#![feature(impl_trait_in_fn_trait_return)]
#![allow(non_snake_case)]
#![allow(clippy::empty_enums)]

use leptos::prelude::*;

mod components;
mod game;
mod header;
mod layout;
mod mobile_controls;
mod modals;
mod sidebar;
mod state;
mod states;

pub type State = &'static state::PlayState;

pub fn state() -> State {
    expect_context::<State>()
}

fn game() -> String {
    leptos_router::hooks::use_params_map()
        .get()
        .get("game")
        .unwrap()
}

#[component]
pub fn Redirect() -> impl IntoView {
    let id = game();
    view! { <leptos_router::components::Redirect path=format!("/{id}/") /> }
}

#[component]
pub fn Play() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::ServerConfiguration>>();
    let motd = config.motd.clone().map(|motd| {
        view! {
            <pre>
                <code>{motd}</code>
            </pre>
        }
    });

    let game_id: std::sync::Arc<str> = game().into();
    let Some(config) = state::config::Builder::new(&config).with_game(game_id.clone()) else {
        return view! { <leptos_router::components::Redirect path="/" /> }.into_any();
    };

    let permission = config.current_game.permission;

    view! {
        <leptos_meta::Link rel="stylesheet" href="/css/play.css" />
        <leptos_meta::Link rel="stylesheet" href="/css/themes.css" />
        <leptos_meta::Title text=format!("{} Online - YNOproject", config.current_game.name) />
        <leptos_meta::Meta
            name="description"
            content=format!(
                "Play multiplayer {} for free! Ad-free and no registration required.",
                config.current_game.name,
            )
        />

        <state::Provider game_id config>
            <layout::Layout />
            <mobile_controls::MobileControls />
            <modals::Modals />
        </state::Provider>

        <PermissionDisclaimer permission />
        {motd}
    }
    .into_any()
}

#[component]
fn PermissionDisclaimer(permission: common::config::PermissionStatus) -> impl IntoView {
    use common::config::PermissionStatus;
    let text = match permission {
        PermissionStatus::Yume1kki => "Pending approval from developer/publisher",
        PermissionStatus::Yume2kki => "Hosted with permission from the Yume 2kki developers",
        // PermissionStatus::CU => "Original disappointment by the YNOproject community",
        PermissionStatus::Pending => "Hosted with permission from the developer(s)",
    };
    view! { <div>{text}</div> }
}
