use leptos::{ev, prelude::*};

#[island]
pub fn Handler() -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::State>>().unwrap();

    window_event_listener(ev::message, move |ev| {
        let Some(message) = common::PlayMessage::de(ev.data()) else {
            return;
        };

        handle(&state, message);
    });
}

fn handle(state: &crate::State, message: common::PlayMessage) {
    match message {
        common::PlayMessage::ConnectionStatusUpdated(status) => {
            state.engine.set_status(status);
        }
    }
}
