use leptos::prelude::*;

mod destination;
mod text_box;

#[component]
pub fn ChatInput() -> impl IntoView {
    view! {
        <div>
            <destination::Destination />
            <text_box::TextBox />
        </div>
    }
}
