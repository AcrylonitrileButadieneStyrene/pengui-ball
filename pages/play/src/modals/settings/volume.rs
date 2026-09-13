use leptos::prelude::*;

#[component]
pub fn Volume() -> impl IntoView {
    view! {
        <label>
            <span>Sound Volume</span>
            <Slider is_music=false />
        </label>
        <label>
            <span>Music Volume</span>
            <Slider is_music=true />
        </label>
    }
}

#[island]
fn Slider(is_music: bool) -> impl IntoView {
    let state = crate::state();

    let on_change = move |event| {
        let Ok(value) = event_target_value(&event).parse() else {
            return;
        };

        let message = if is_music {
            common::EngineMessage::SetMusicVolume(value)
        } else {
            common::EngineMessage::SetSoundVolume(value)
        };

        state.engine.send(message);
    };

    view! { <input type="range" min=1 max=100 value=100 on:input=on_change /> }
}
