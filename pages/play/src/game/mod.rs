use leptos::prelude::*;

mod controls;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Game() -> impl IntoView {
    view! {
        <div class=style::game_window>
            <controls::Controls />
            <Engine />
        </div>
    }
}

#[island]
fn Engine() -> impl IntoView {
    let state = expect_context::<std::sync::Arc<crate::State>>();

    view! {
        <iframe
            node_ref=state.engine.frame
            class=style::player
            src="./engine"
            title="Game Engine"
        />
    }
}
