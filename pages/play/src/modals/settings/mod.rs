use leptos::prelude::*;

mod nametags;
mod volume;

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::Settings>
            <volume::Volume />
            <nametags::NameTags />
        </super::Modal>
    }
}
