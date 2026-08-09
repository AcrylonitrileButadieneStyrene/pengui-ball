use leptos::prelude::*;

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::Settings>
            <Container></Container>
        </super::Modal>
    }
}

#[island]
pub fn Container() -> impl IntoView {
    let state = crate::state();
    let (music_volume, set_music_volume) = signal(100);
    let (sound_volume, set_sound_volume) = signal(100);
    Effect::new(move || {
        state.engine.send(common::EngineMessage::SetVolumes {
            music: music_volume.get(),
            sound: sound_volume.get(),
        });
    });
    view! {
        <label for="sound-volume">Sound Volume</label>
        <input
            id="sound-volume"
            type="range"
            min=1
            max=100
            prop:value=sound_volume
            on:input:target=move |ev| set_sound_volume(ev.target().value().parse().unwrap())
        />
        <label for="music-volume">Music Volume</label>
        <input
            id="music-volume"
            type="range"
            min=1
            max=100
            prop:value=music_volume
            on:input:target=move |ev| set_music_volume(ev.target().value().parse().unwrap())
        />
    }
}
