use leptos::prelude::*;

stylance::import_style!(pub style, "logo.module.css");

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <a href="/" class=style::logo>
            <svg height="48" viewBox="0 0 64 28">
                <path d="m0 0h6v10h16v-10h6v16h-11v12.5h-6v-12.5h-11v-15.5m34-0.5h22v6h-16v22.5h-6v-28.5m22 6h6v22.5h-6v-22.5" />
            </svg>
            <svg height="48" viewBox="0 0 28 28">
                <path
                    class=style::closed
                    d="m0 0h28v28h-28v-28m10 13h-2v2h2v-2m-4 9h16v-16h-16v16m0.5-15.5h15v15h-15v-15"
                />
                <path
                    class=style::open
                    d="m0 0h28v28h-10l-6 7v-21l9-9h-15v17h3v6h-9v-28m22 5h-0.5v17h0.5v-17m-6 15h-1.5v0.5h-0.5v1.5h2v-2"
                />
            </svg>
        </a>
    }
}
