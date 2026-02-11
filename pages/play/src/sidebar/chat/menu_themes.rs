use leptos::prelude::*;

#[component]
pub fn MenuThemes() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::ServerConfiguration>>();
    let lines = config
        .menu_themes
        .iter()
        .map(|(name, props)| format!("--{}-{name}-gradient: {};", props.game, props.gradient))
        .collect::<String>();

    view! { <style>{format!(":root {{{lines}}}")}</style> }
}
