use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

stylance::import_style!(pub style, "private.module.css");

#[island]
pub fn Private(children: Children) -> impl IntoView {
    let state = crate::state();
    let config = state.config.game;

    let on_click =
        move |_| config.update(|config| config.private_mode = (config.private_mode + 1) % 3);

    let mut last_sent = None;
    Effect::new(move || {
        if state.session.status.get() != ConnectionReadyState::Open {
            last_sent = None;
            return;
        }

        let mode = config.get().private_mode;
        if Some(mode) == last_sent {
            return;
        }

        last_sent = Some(mode);
        let message = crate::sidebar::session::Command::PrivateMode(mode);
        state.session.channel.send(message).unwrap();
        state.engine.send(common::EngineMessage::Connect);
    });

    view! {
        <button
            class=style::button
            class:private=move || config.get().private_mode == 1
            class:single=move || config.get().private_mode == 2
            on:click=on_click
        >
            {children()}
        </button>
    }
}
