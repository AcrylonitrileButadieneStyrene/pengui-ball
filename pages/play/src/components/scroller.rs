use leptos::prelude::*;

stylance::import_style!(pub style, "scroller.module.css");

#[component]
pub fn Scroller(children: Children) -> impl IntoView {
    view! {
        <div class=style::scroller>
            {children()}
        </div>
    }
}
