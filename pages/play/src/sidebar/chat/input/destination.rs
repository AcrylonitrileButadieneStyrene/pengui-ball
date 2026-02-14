use leptos::prelude::*;
use strum::{EnumProperty, VariantArray};

use crate::state::chat::MessageDestination;

stylance::import_style!(pub style, "destination.module.css");

#[component]
pub fn Destination() -> impl IntoView {
    view! {
        <div class=style::destination>
            <span>"Send to "</span>
            <Selection>
                <option value=-1 style:display="none">
                    "nowhere"
                </option>
            </Selection>
        </div>
    }
}

#[island]
fn Selection(children: Children) -> impl IntoView {
    let state = crate::state();
    let destination = state.chat.destination;

    let (none, set_none) = signal(false);

    let on_change = move |event| {
        let value = event_target_value(&event);
        if let Ok(repr) = value.parse::<u8>()
            && let Some(dest) = MessageDestination::from_repr(repr)
        {
            destination.set(dest);
        }
    };

    let channels = MessageDestination::VARIANTS
        .iter()
        .map(|variant| (*variant, variant.to_channel(&state.chat).filter))
        .collect::<Vec<(MessageDestination, RwSignal<bool>)>>();

    let options = channels
        .iter()
        .map(move |(variant, filter)| {
            view! {
                <option value=*variant as u8 prop:disabled=*filter>
                    {variant.get_str("Name")}
                </option>
            }
        })
        .collect::<Vec<_>>();

    Effect::new(move || {
        let dest = destination.get();

        let filtered = channels[dest as usize].1.get();
        if !filtered {
            set_none(false);
            return;
        }

        for (index, filter) in channels.iter().enumerate() {
            if index == dest as usize {
                continue;
            }

            if !filter.1.get() {
                destination.set(MessageDestination::from_repr(index.try_into().unwrap()).unwrap());
                set_none(false);
                return;
            }
        }

        set_none(true);
    });

    let value = move || {
        if none() { -1 } else { destination.get() as _ }
    };

    view! {
        <select prop:disabled=none on:change=on_change prop:value=value>
            {children()}
            {options}
        </select>
    }
}
