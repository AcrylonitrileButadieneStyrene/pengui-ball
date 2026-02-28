use leptos::prelude::*;

stylance::import_style!(pub style, "layout.module.css");

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <Main {..} id="layout" class=style::layout>
            <super::header::Header />
            <super::game::Game />
            <super::sidebar::Sidebar />
        </Main>
    }
}

#[island]
fn Main(children: Children) -> impl IntoView {
    let state = crate::state();
    let chat_hidden = move || state.config.game.read().chat_hidden;

    view! { <main class:chat-hidden=chat_hidden>{children()}</main> }
}
