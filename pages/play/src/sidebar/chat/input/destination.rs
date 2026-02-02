use leptos::prelude::*;
use strum::{EnumProperty, VariantArray};

use crate::state::chat::MessageDestination;

stylance::import_style!(pub style, "destination.module.css");

#[component]
pub fn Destination() -> impl IntoView {
    view! {
        <div>
            <span>"Send to "</span>
            <Selection />
        </div>
    }
}

#[island]
fn Selection() -> impl IntoView {
    let state = crate::state();
    let destination = state.chat.destination;

    let on_change = move |event| {
        let value = event_target_value(&event);
        if let Ok(repr) = value.parse::<u8>()
            && let Some(dest) = MessageDestination::from_repr(repr)
        {
            destination.set(dest);
        }
    };

    let options = MessageDestination::VARIANTS
        .iter()
        .map(|variant| {
            view! { <option value=*variant as u8>{variant.get_str("Name")}</option> }
        })
        .collect::<Vec<_>>();

    view! {
        <select
            class=style::selection
            prop:value=move || destination.get() as u8
            on:change=on_change
        >
            // the options must be inside of the island due to a hydration error
            {options}
        </select>
    }
}
