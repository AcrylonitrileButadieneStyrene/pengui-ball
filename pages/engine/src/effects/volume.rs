use leptos::prelude::*;

pub fn effect(state: std::sync::Arc<crate::EngineState>) {
    Effect::new(move || {
        state.easyrpg_player.call({
            let state = state.clone();
            move |engine| {
                let (sound, music) = if state.muted.get() {
                    (0, 0)
                } else {
                    (state.sound_volume.get(), state.music_volume.get())
                };

                let api = engine.api();
                api.set_sound_volume(sound);
                api.set_music_volume(music);
            }
        });
    });
}
