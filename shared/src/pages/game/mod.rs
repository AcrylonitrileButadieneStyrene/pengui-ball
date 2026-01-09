use leptos::prelude::*;

mod session;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Game() -> impl IntoView {
    view! {
        <leptos_meta::Body {..} class=style::game />
        <main class=style::main>
            Main
            <div>
                Nav
            </div>
            <div class=style::content>
                <div class=style::canvas_iframe>Canvas</div>
                <div class=style::chat>Chat</div>
            </div>
        </main>
    }
}
