use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

mod controls;
mod messages;

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
    let state = crate::state();
    let frame = state.engine.frame;
    let status = state.session.status;
    messages::setup_handler(state);

    Effect::new(move || {
        if status.get() == ConnectionReadyState::Open {
            crate::state::EngineState::send_frame(frame, common::EngineMessage::Connect);
        }
    });

    view! {
        <iframe
            node_ref=frame
            class=style::player
            src="./engine"
            title="Game Engine"
        />
    }
}
