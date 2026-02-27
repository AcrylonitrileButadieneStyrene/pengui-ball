use leptos::prelude::*;

mod author;
mod icons;
pub mod components;

stylance::import_style!(pub style, "mod.module.css");

#[component]
fn Message(
    #[prop(into)] filtered: Signal<bool>,
    #[prop(optional, into)] header: ViewFnOnce,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=style::message
            style:display=move || { if filtered.get() { "none" } else { "" } }
        >
            <div class=style::header>{header.run()}</div>
            <div>{children()}</div>
        </div>
    }
}
