use leptos::prelude::*;

#[component]
pub fn modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::CORS>
            CORS issue. I will make a userscript for this later.<br />
            For now use a CORS bypass extension
            (and configure it correctly for your own security).
        </super::Modal>
    }
}
