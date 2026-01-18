use leptos::prelude::*;

stylance::import_style!(pub style, "player_count.module.css");

#[island]
pub fn PlayerCount() -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::state::State>>().unwrap();

    let text = move || {
        format!(
            "{} Players Online",
            state
                .player_count
                .get()
                .map_or_else(|| "?".to_string(), |count| count.to_string())
        )
    };

    view! { <div class=style::counter>{text}</div> }
}
