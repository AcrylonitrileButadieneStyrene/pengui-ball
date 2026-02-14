use leptos::{ev, prelude::*};

pub fn setup_handler(state: crate::EngineState) {
    window_event_listener(ev::message, move |ev| {
        let Some(message) = common::EngineMessage::de(ev.data()) else {
            return;
        };

        handle(&state, message);
    });
}

fn handle(state: &crate::EngineState, message: common::EngineMessage) {
    match message {
        common::EngineMessage::Connect => {
            state
                .easyrpg_player
                .call_untracked(|player| player.api().session_ready());
        }
        common::EngineMessage::Mute(muted) => {
            state.muted.set(muted);
        }
        common::EngineMessage::Defocus => {
            if document().has_focus().unwrap_or_default() {
                return;
            }

            let _ = window()
                .dispatch_event(&leptos::web_sys::Event::new("blur").unwrap())
                .unwrap();
        }
    }
}

pub fn send(message: common::PlayMessage) {
    window()
        .parent()
        .unwrap()
        .unwrap()
        .post_message(&message.ser(), "*")
        .unwrap();
}
