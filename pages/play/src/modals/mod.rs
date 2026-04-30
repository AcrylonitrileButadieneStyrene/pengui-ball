use leptos::prelude::*;

pub mod cors;
pub mod expeds;
pub mod login;
pub mod logout;
pub mod saves;
pub mod screenshots;
pub mod themes;

stylance::import_style!(pub style, "mod.module.css");

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Modals {
    Cors,
    Expeds,
    LogIn,
    LogOut,
    Saves,
    Screenshots,
    Themes,
}

#[component]
pub fn Modals() -> impl IntoView {
    view! {
        <cors::Modal />
        <expeds::Modal />
        <login::Modal />
        <logout::Modal />
        <saves::Modal />
        <screenshots::Modal />
        <themes::Modal />
    }
}

#[island]
pub fn Modal(when: Modals, children: Children) -> impl IntoView {
    let state = crate::state();
    let node_ref = NodeRef::new();

    let on_close = {
        let state = state.clone();
        move |_| state.modal.set(None)
    };

    // if only there was a way to open a dialog modally without code
    Effect::new(move || {
        let Some::<leptos::web_sys::HtmlDialogElement>(node) = node_ref.get() else {
            return;
        };

        let which = state.modal.get();
        if which == Some(when) {
            // the result just indicates if the dialog was opened non-modally
            // which it never will be unless someone's script messes with it
            // https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/showModal
            let _ = node.show_modal();
        } else {
            node.close();
        }
    });

    view! {
        <dialog
            node_ref=node_ref
            class=style::modal
            on:close=on_close
            autofocus=true
            closedby="any"
        >
            <form method="dialog">
                <button>{"\u{2716}"}</button>
            </form>
            {children()}
        </dialog>
    }
}
