use leptos::prelude::*;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Game() -> impl IntoView {
    let game = use_context::<crate::CurrentGame>().unwrap();

    view! {
        <div class=style::game_window>
            <div style="height: 32px; background-color: gray;" />
            <Engine id=game.id.clone() />
        </div>
    }
}

#[island]
fn Engine(id: String) -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::State>>().unwrap();

    view! {
        <iframe
            node_ref=state.engine.frame
            class=style::player
            src=format!("/engine?game={id}")
            title="Game Engine"
        />
    }
}
