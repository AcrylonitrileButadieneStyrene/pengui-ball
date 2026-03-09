use std::sync::Arc;

use common::PlayMessage;
use leptos::{ev, prelude::*};
use leptos_use::core::ConnectionReadyState;

mod player;
mod save;

pub fn setup_handler(state: Arc<crate::state::PlayState>) {
    window_event_listener(ev::message, move |ev| {
        let Some(message) = common::PlayMessage::de(ev.data()) else {
            return;
        };

        handle(&state, message);
    });
}

fn handle(state: &crate::state::PlayState, message: common::PlayMessage) {
    match message {
        PlayMessage::EngineLoaded => {
            state.engine.load_count.update(|count| *count += 1);
            if state.session.status.get_untracked() == ConnectionReadyState::Open {
                state.engine.send(common::EngineMessage::Connect);
            }
        }
        PlayMessage::ConnectionStatusUpdated(status) => {
            state.engine.set_status(status);
        }
        PlayMessage::PlayerSync(data) => player::sync(state, data),
        PlayMessage::PlayerConnect(data) => player::connect(state, data),
        PlayMessage::PlayerDisconnect(id) => player::disconnect(state, id),
        PlayMessage::PlayerTeleported(map, x, y) => player::teleported(state, map, x, y),
        PlayMessage::PlayerSpriteUpdated(id, charset, index) => {
            player::sprite_update(state, id, charset, index)
        }
        PlayMessage::RegainFocus(_shift_held) => {
            if let Some(element) = state.chat.input.get_untracked() {
                element.focus().unwrap();
            }
        }
        PlayMessage::SaveData(slot, save_file) => {
            save::data(state, slot, save_file);
        }
        PlayMessage::SaveTimestamps(timestamps) => {
            state.engine.save_timestamps.set(timestamps);
        }
        PlayMessage::RoomSwitch => state.players.in_map.update(|uuids| uuids.clear()),
    }
}
