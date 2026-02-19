use leptos::prelude::*;

pub fn effect(state: crate::EngineState) {
    Effect::new(move || {
        let game = state.game.clone();
        leptos::task::spawn_local(async move {
            let timestamps = crate::easyrpg::files::get_timestamps(game.clone()).await;
            crate::send(common::PlayMessage::SaveTimestamps(timestamps));
        });
    });
}
