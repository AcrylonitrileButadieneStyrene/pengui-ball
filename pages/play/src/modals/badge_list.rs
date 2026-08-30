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
            .by_group
            .get()
            .get("2kki")
            .and_then(|game| game.get(&Some("4_ch".into())).cloned())
            .unwrap_or_default()
            .into_iter()
            .map(|badge| badge.badge_id.to_string())
            .collect::<Vec<_>>()
    }
}
