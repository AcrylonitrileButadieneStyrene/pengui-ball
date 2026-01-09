use leptos::prelude::*;

mod session;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Game() -> impl IntoView {
    view! {
        <leptos_meta::Body {..} class=style::game />

        <div class=style::horizontal_box>
            <header class=style::header class=(style::border, true)>
                <div style="height: 60px; background-color: white;" />
            </header>

            <main class=style::main>
                <div class=style::canvas_iframe class=(style::border, true)>
                    <div style="height: 32px; background-color: gray;" />
                    <div style="width: 640px; height: 480px; background-color: pink;" />
                </div>
                <div class=style::chat class=(style::border, true)>
                    <div style="width: 284px; height: 100%; background-color: darkgreen;" />
                </div>
            </main>
        </div>
    }
}
