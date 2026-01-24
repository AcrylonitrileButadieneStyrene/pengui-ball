use leptos::prelude::*;

pub mod cors;

stylance::import_style!(pub style, "mod.module.css");

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Modals {
    CORS,
}

#[component]
pub fn Modals() -> impl IntoView {
    view! { <cors::Modal /> }
}

#[island]
pub fn Modal(when: Modals, children: Children) -> impl IntoView {
    let state = crate::state();

    let on_close = {
        let state = state.clone();
        move |_| state.modal.set(None)
    };

    view! {
        <dialog
            class=style::modal
            prop:open=move || state.modal.get() == Some(when)
            on:close=on_close
        >
            <form method="dialog">
                <button>{"\u{2716}"}</button>
            </form>
            {children()}
        </dialog>
    }
}
