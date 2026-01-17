use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod session;
mod state;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Game() -> impl IntoView {
    let id = use_params_map().get().get("game").unwrap();
    let config = use_context::<std::sync::Arc<crate::Config>>().unwrap();
    let games = config.games.clone();

    let Some(game) = games.into_iter().find(|game| game.id == id) else {
        return leptos::either::Either::Left(view! { <leptos_router::components::Redirect path="/" /> });
    };

    leptos::either::Either::Right(view! {
        <leptos_meta::Title text=format!("{} Online - YNOproject", game.name) />
        <leptos_meta::Body {..} class=style::game />

        <state::Provider>
            <main class=style::main>
                <header class=style::header class=(style::border, true)>
                    <div style="height: 60px; background-color: white;" />
                </header>

                <div class=style::game_window class=(style::border, true)>
                    <div style="height: 32px; background-color: gray;" />
                    <iframe
                        class=style::player
                        src=format!("/player?game={}", game.id)
                        title="Game Engine"
                    />
                </div>

                <div class=style::chat class=(style::border, true)>
                    <div style="width: 100%; height: 100%; background-color: darkgreen;" />
                </div>
            </main>
        </state::Provider>
    })
}
