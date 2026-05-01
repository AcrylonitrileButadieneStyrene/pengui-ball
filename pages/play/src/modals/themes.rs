use std::sync::Arc;

use leptos::prelude::*;

stylance::import_style!(pub style, "themes.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::ServerConfiguration>>();
    let game = expect_context::<crate::CurrentGame>();
    let themes = config.themes.get(&game.id).cloned();

    let render = move |themes| {
        view! {
            <div class=style::container>
                <Inner game=game.id.clone() themes />
            </div>
        }
        .into_any()
    };

    view! {
        <super::Modal when=super::Modals::Themes>
            <h1>UI Theme</h1>
            {themes.map_or_else(Fallback, render)}
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
            view! { <Icon game=game.clone() theme /> }
        })
        .collect::<Vec<_>>()
}

#[component]
fn Icon(game: Arc<str>, theme: Arc<str>) -> impl IntoView {
    let base = format!("/_yno/{game}/images/ui/{game}/{theme}");

    view! {
        <div
            class=style::icon
            style=("--bg", format!("url('{base}/containerbg.png')"))
            style=("--bd", format!("url('{base}/border.png')"))
        >
            <img loading="lazy" src=format!("{base}/arrowup.png") />
            <img loading="lazy" src=format!("{base}/arrowdown.png") />
        </div>
    }
}
