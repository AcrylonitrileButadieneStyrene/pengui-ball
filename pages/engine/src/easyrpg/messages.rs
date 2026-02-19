use common::EngineMessage;
use leptos::{ev, prelude::*};

pub fn setup_handler(state: crate::EngineState) {
    window_event_listener(ev::message, move |ev| {
        let Some(message) = EngineMessage::de(ev.data()) else {
            return;
        };

        handle(&state, message);
    });
}

fn handle(state: &crate::EngineState, message: EngineMessage) {
    match message {
        EngineMessage::Connect => {
            state
                .easyrpg_player
                .call_untracked(|player| player.api().session_ready());
        }
        EngineMessage::Mute(muted) => {
            state.muted.set(muted);
        }
        EngineMessage::Focus(active) => {
            if document().has_focus().unwrap_or_default() {
                return;
            }

            crate::effects::events::control_timer(state.defocus_timeout, !active);
        }
        EngineMessage::SetSave(id, data) => super::files::set_file(state.game.clone(), id, data),
        EngineMessage::GetSave(id) => super::files::get_file(state.game.clone(), id),
        EngineMessage::DeleteSave(id) => super::files::delete_file(state.game.clone(), id),
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
