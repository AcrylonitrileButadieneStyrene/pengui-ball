use leptos::prelude::*;

#[island]
pub fn OpenModal(modal: crate::modals::Modals, children: Children) -> impl IntoView {
    let state = crate::state();

    view! {
        <button on:click=move |_| state.modal.set(Some(modal)) class="pop-out">
            {children()}
        </button>
    }
}
