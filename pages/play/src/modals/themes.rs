use std::sync::Arc;

use leptos::prelude::*;

#[component]
pub fn Modal() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::ServerConfiguration>>();
    let game = expect_context::<crate::CurrentGame>();
    let themes = config.themes.get(&game.id).cloned();

    view! {
        <super::Modal when=super::Modals::Themes>
            {themes.map_or_else(Fallback, |themes| view!{ <Inner game=game.id.clone() themes /> }.into_any())}
        </super::Modal>
    }
}

fn Fallback() -> AnyView {
    "This game does not yet support menu themes".into_any()
}

#[island]
fn Inner(game: Arc<str>, themes: Vec<Arc<str>>) -> impl IntoView {
    themes
        .into_iter()
        .map(|theme| {
            view! {
                <div>{theme}</div>
            }
        })
        .collect::<Vec<_>>()
}
