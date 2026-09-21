use leptos::prelude::*;
use strum::VariantArray as _;

use crate::state::chat::MessageDestination;

stylance::import_style!(pub style, "channels.module.css");

#[component]
pub fn Channels() -> impl IntoView {
    view! {
        <div class=style::container>
            <Channel of=MessageDestination::Map />
            <Channel of=MessageDestination::Global />
            <Channel of=MessageDestination::Party />
        </div>
    }
}

#[component]
fn Channel(of: MessageDestination) -> impl IntoView {
    let label: &'static str = of.into();
    view! { <label>{label} <Handler of /></label> }
}

#[island]
fn Handler(of: MessageDestination) -> impl IntoView {
    let state = crate::state();
    let visible = of.to_channel(&state.chat).visible;
    let destination = state.chat.destination;

    let on_change = move |event: leptos::ev::Event| {
        let input = event_target::<leptos::web_sys::HtmlInputElement>(&event);

        let checked = input.checked();
        let active = destination.get_untracked() == Some(of);

        if !checked && active {
            visible.set(false);

            for variant in MessageDestination::VARIANTS {
                if *variant == of {
                    continue;
                }

                if variant.to_channel(&state.chat).visible.get_untracked() {
                    destination.set(Some(*variant));
                    return;
                }
            }

            destination.set(None);
        } else {
            visible.set(true);
            destination.set(Some(of));
        }
    };

    let is_checked = move || destination.get() == Some(of);
    view! {
        <input type="checkbox" prop:checked=visible on:change=on_change />
        <input type="radio" name="chat-destination" prop:checked=is_checked />
    }
}
