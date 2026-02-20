use leptos::prelude::*;
use leptos_use::storage::{UseStorageOptions, use_local_storage_with_options};

#[island]
pub fn Mute(children: Children) -> impl IntoView {
    let state = crate::state();
    let (muted, set_muted, _) = use_local_storage_with_options::<_, codee::string::FromToStringCodec>(
        "engine-muted",
        UseStorageOptions::default().delay_during_hydration(true),
    );

    Effect::new(move || {
        state.engine.load_count.track();
        state.engine.send(common::EngineMessage::Mute(muted()));
    });

    let on_click = move |_| set_muted(!muted.get_untracked());

    view! {
        <button class:stricken=muted on:click=on_click>
            {children()}
        </button>
    }
}
