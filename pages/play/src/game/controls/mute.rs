use leptos::prelude::*;

#[island]
pub fn Mute(children: Children) -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::State>>().unwrap();
    let (muted, set_muted) = signal(false);

    Effect::new(move || {
        state.engine.send(common::EngineMessage::Mute(muted()));
    });

    let on_click = move |_| set_muted(!muted.get_untracked());

    view! {
        <button class:stricken=muted on:click=on_click>
            {children()}
        </button>
    }
}
