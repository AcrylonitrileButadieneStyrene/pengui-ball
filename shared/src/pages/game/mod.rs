use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

mod session;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Game() -> impl IntoView {
    let game = use_params_map().get().get("game").unwrap();

    view! {
        <leptos_meta::Body {..} class=style::game />

        <div class=style::horizontal_box>
            <header class=style::header class=(style::border, true)>
                <div style="height: 60px; background-color: white;" />
            </header>

            <main class=style::main>
                <div class=style::game_window class=(style::border, true)>
                    <div style="height: 32px; background-color: gray;" />
                    <iframe class=style::player src=format!("/player?game={game}")/>
                </div>
                <div class=style::chat class=(style::border, true)>
                    <div style="width: 284px; height: 100%; background-color: darkgreen;" />
                </div>
            </main>
        </div>
    }
}
