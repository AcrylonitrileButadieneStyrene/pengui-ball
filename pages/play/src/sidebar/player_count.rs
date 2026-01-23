use leptos::prelude::*;

stylance::import_style!(pub style, "player_count.module.css");

#[island]
pub fn PlayerCount() -> impl IntoView {
    let state = crate::state();
    let text = move || {
        format!(
            "{} Players Online",
            state
                .players
                .count
                .get()
                .map_or_else(|| "?".to_string(), |count| count.to_string())
        )
    };

    view! { <div class=style::counter>{text}</div> }
}
