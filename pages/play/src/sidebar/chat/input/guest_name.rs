use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

use crate::{sidebar::session::Command, states::players::player::PlayerStoreFields};

stylance::import_style!(pub style, "guest_name.module.css");

#[component]
pub fn GuestName() -> impl IntoView {
    view! {
        <div class=style::container>
            <div>You must set a nickname before you can chat.</div>
            <div>* Maximum 10 characters</div>
            <div>* Alphanumeric characters only</div>
            <Input />
        </div>
    }
}

#[island]
fn Input() -> impl IntoView {
    let state = crate::state();
    let guest_name = state.players.local.name();

    let on_keydown = move |event: leptos::ev::KeyboardEvent| {
        if event.key() != "Enter" {
            return;
        }

        let value = event_target_value(&event);
        guest_name.set(Some(value.into()));
    };

    Effect::new(move || {
        if let Some(guest_name) = guest_name.get()
            && state.session.status.get() == ConnectionReadyState::Open
        {
            state
                .session
                .channel
                .send(Command::SetName(guest_name.to_string()))
                .unwrap();
        }
    });

    view! { <input on:keydown=on_keydown /> }
}
