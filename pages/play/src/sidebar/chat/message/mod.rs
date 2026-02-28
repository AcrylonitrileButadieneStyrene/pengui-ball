use leptos::prelude::*;

mod author;
pub mod components;
mod icons;

stylance::import_style!(pub style, "mod.module.css");

#[component]
fn Message(
    #[prop(into)] filtered: Signal<bool>,
    #[prop(optional, into)] header: ViewFnOnce,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=style::message prop:hidden=filtered>
            <div class=style::header>{header.run()}</div>
            <div>{children()}</div>
        </div>
    }
}
