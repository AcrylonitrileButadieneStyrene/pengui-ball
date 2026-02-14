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
    let load_count = state.engine.load_count;
    messages::setup_handler(state);

    Effect::new(move || {
        if status.get() == ConnectionReadyState::Open {
            crate::state::engine::State::send_frame(frame, common::EngineMessage::Connect);
        }
    });

    window_event_listener(leptos::ev::blur, move |_| {
        crate::state::engine::State::send_frame(frame, common::EngineMessage::Defocus);
    });

    view! {
        <iframe
            node_ref=frame
            class=style::player
            src="./engine"
            title="Game Engine"
            on:load=move |_| load_count.update(|val| *val += 1)
        />
    }
}
