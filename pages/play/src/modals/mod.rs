use leptos::prelude::*;

pub mod cors;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Modal(children: Children) -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <dialog class=style::modal prop:open=visible>
            <button on:click=move |_| set_visible(false)>{"\u{2716}"}</button>
            {children()}
        </dialog>
    }
}
