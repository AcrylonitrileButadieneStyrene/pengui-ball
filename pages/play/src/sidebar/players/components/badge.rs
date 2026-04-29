use leptos::prelude::*;

#[component]
pub fn Badge(badge: Option<std::sync::Arc<str>>) -> impl IntoView {
    Some(view! {
        <img
            class=super::style::badge
            loading="lazy"
            src=format!("https://ynoproject.net/2kki/images/badge/{}.png", badge?)
        />
    })
}
