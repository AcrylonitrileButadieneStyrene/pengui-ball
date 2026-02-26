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
    .into_any()
}

#[island]
fn Engine() -> impl IntoView {
    let state = crate::state();
    let frame = state.engine.frame;
    let status = state.session.status;
    messages::setup_handler(state);

    Effect::new(move || {
        if status.get() == ConnectionReadyState::Open {
            crate::state::engine::State::send_frame(frame, common::EngineMessage::Connect);
        }
    });

    window_event_listener(leptos::ev::focus, move |_| {
        crate::state::engine::State::send_frame(frame, common::EngineMessage::Focus(true));
    });

    window_event_listener(leptos::ev::blur, move |_| {
        crate::state::engine::State::send_frame(frame, common::EngineMessage::Focus(false));
    });

    // let UseWindowSizeReturn { height, width } = use_window_size();
    // let adaptive_scale = move || {
    //     let remaining_width = width.get() - 384.;
    //     let remaining_height = height.get() - 192.;
    //     (remaining_width / 320.)
    //         .min(remaining_height / 240.)
    //         .trunc()
    //         .to_string()
    // };

    view! { <iframe node_ref=frame class=style::player src="./engine" title="Game Engine" /> }
}
