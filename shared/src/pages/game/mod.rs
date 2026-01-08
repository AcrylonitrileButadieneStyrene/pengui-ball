use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::pages::game::session::ProvideSession;

mod session;

#[component]
pub fn Game() -> impl IntoView {
    let params = use_params_map();
    let game = move || params.with(|params| params.get("game").unwrap_or_default());

    let handle = window_event_listener(leptos::ev::message, |msg| leptos::logging::log!("{msg:?}"));
    on_cleanup(|| handle.remove());

    view! {
        <ProvideSession game=game()>
            <p>The game is {game}</p>
            <iframe src=format!("player?game={}", game()) width=320 height=240 />
        </ProvideSession>
    }
}
