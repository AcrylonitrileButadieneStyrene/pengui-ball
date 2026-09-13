use std::str::FromStr as _;

use leptos::prelude::*;

#[derive(Clone, Copy, Default, strum::IntoStaticStr, strum::EnumString)]
#[repr(u8)]
enum NameTagMode {
    None = 0,
    #[default]
    Classic,
    Compact,
    Slim,
}

#[island]
pub fn NameTags() -> impl IntoView {
    let state = crate::state();
    let (selected, set_selected) = signal(NameTagMode::default());

    let on_change = move |event| {
        let value = event_target_value(&event);
        if let Ok(mode) = NameTagMode::from_str(&value) {
            set_selected(mode);
        }
    };

    Effect::new(move || {
        state.engine.load_count.track();
        state
            .engine
            .send(common::EngineMessage::SetNameTagMode(selected() as u8));
    });

    view! {
        <label>
            <span>Nametags</span>
            <select on:change=on_change prop:value=move || to_string(selected())>
                <option value="None">None</option>
                <option value="Classic" selected>Classic</option>
                <option value="Compact">Compact</option>
                <option value="Slim">Slim</option>
            </select>
        </label>
    }
}

fn to_string(value: NameTagMode) -> &'static str {
    value.into()
}
