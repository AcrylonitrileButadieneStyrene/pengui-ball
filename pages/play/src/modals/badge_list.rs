use leptos::prelude::*;

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::BadgeList>
            <div style:height="10000px">
                <Inner/>
            </div>
        </super::Modal>
    }
}

#[island]
fn Inner() -> impl IntoView {
    let state = crate::state();

    move || {
        state
            .badges
            .games
            .get()
            .get("2kki")
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|badge| badge.badge_id.to_string())
            .collect::<Vec<_>>()
    }
}
