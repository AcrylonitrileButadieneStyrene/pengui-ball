use std::sync::Arc;

use leptos::{prelude::*, wasm_bindgen::JsCast as _, web_sys};

stylance::import_style!(pub style, "themes.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::ServerConfiguration>>();
    let game = expect_context::<crate::CurrentGame>();
    let themes = config.themes.get(&game.id).cloned();

    let render = move |themes: Vec<_>| {
        let icons = themes
            .into_iter()
            .map(|theme| view! { <Icon game=game.id.clone() theme /> })
            .collect::<Vec<_>>();

        view! { <Listener>{icons}</Listener> }.into_any()
    };

    view! {
        <super::Modal when=super::Modals::Themes>
            <h1>UI Theme</h1>
            {themes.map_or_else(Fallback, render)}
        </super::Modal>
    }
}

#[island]
fn Listener(children: Children) -> impl IntoView {
    let on_click = |event: leptos::ev::MouseEvent| {
        if let Some(target) = event.target()
            && let Some(element) = target.dyn_ref::<web_sys::HtmlButtonElement>()
            && let Some(id) = element.dataset().get("id")
        {
            leptos::logging::log!("changing to {id}");
        }
    };

    view! {
        <div class=style::container on:click=on_click>
            {children()}
        </div>
    }
}

fn Fallback() -> AnyView {
    "This game does not yet support menu themes".into_any()
}

#[component]
fn Icon(game: Arc<str>, theme: Arc<str>) -> impl IntoView {
    let base = format!("/_yno/{game}/images/ui/{game}/{theme}");

    view! {
        <button
            class=format!("{} button pop-out", style::icon)
            data-id=theme
            style=("--bg", format!("url('{base}/containerbg.png')"))
            style=("--bd", format!("url('{base}/border.png')"))
        >
            <img loading="lazy" src=format!("{base}/arrowup.png") />
            <img loading="lazy" src=format!("{base}/arrowdown.png") />
        </button>
    }
}
