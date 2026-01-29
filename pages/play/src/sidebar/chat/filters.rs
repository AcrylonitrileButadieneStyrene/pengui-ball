use leptos::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
enum Filter {
    Map,
    Global,
    Party,
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Map => f.write_str("Map"),
            Self::Global => f.write_str("Global"),
            Self::Party => f.write_str("Party"),
        }
    }
}

stylance::import_style!(pub style, "filters.module.css");

#[component]
pub fn Filters() -> impl IntoView {
    view! {
        <div class=style::filters>
            <Filter of=Filter::Map />
            <Filter of=Filter::Global />
            <Filter of=Filter::Party />
        </div>
    }
}

#[component]
fn Filter(of: Filter) -> impl IntoView {
    view! { <label role="button">{of.to_string()} <Handler filter=of /></label> }
}

#[island]
fn Handler(filter: Filter) -> impl IntoView {
    let state = crate::state();
    let filter = match filter {
        Filter::Map => state.chat.map.filter,
        Filter::Global => state.chat.global.filter,
        Filter::Party => state.chat.party.filter,
    };

    let on_change = move |ev: leptos::ev::Event| {
        let input = event_target::<leptos::web_sys::HtmlInputElement>(&ev);
        let state = input.checked();
        filter.set(!state);
    };

    view! { <input type="checkbox" prop:checked=move || !filter.get() on:change=on_change /> }
}
