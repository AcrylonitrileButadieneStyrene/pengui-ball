use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod sidebar;
mod state;

stylance::import_style!(pub style, "mod.module.css");

pub type CurrentGame = std::sync::Arc<common::config::Game>;

#[component]
pub fn Game() -> impl IntoView {
    let id = use_params_map().get().get("game").unwrap();
    let config = use_context::<std::sync::Arc<common::Config>>().unwrap();
    let games = config.games.clone();

    let Some(game) = games.into_iter().find(|game| game.id == id) else {
        return leptos::either::Either::Left(
            view! { <leptos_router::components::Redirect path="/" /> },
        );
    };

    let game = std::sync::Arc::new(game);
    provide_context(game.clone());

    leptos::either::Either::Right(view! {
        <leptos_meta::Title text=format!("{} Online - YNOproject", game.name) />
        <leptos_meta::Body {..} class=style::game />

        <state::Provider>
            <main class=style::layout>
                <header class=style::header>
                    <div style="height: 60px; background-color: white;" />
                </header>

                <div class=style::game_window>
                    <div style="height: 32px; background-color: gray;" />
                    <iframe
                        class=style::player
                        src=format!("/engine?game={}", game.id)
                        title="Game Engine"
                    />
                </div>

                <div class=style::chat>
                    <sidebar::Sidebar />
                </div>
            </main>
        </state::Provider>
    })
}
