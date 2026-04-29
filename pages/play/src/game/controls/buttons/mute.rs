use leptos::prelude::*;

#[island]
pub fn Mute(children: Children) -> impl IntoView {
    let state = crate::state();
    let game_config = state.config.game;
    let muted = move || game_config.get().muted;

    Effect::new(move || {
        state.engine.load_count.track();
        state.engine.send(common::EngineMessage::Mute(muted()));
    });

    let on_click = move |_| game_config.update(|config| config.muted ^= true);

    view! {
        <button class="strickable pop-out" class:stricken=muted on:click=on_click>
            {children()}
        </button>
    }
}
