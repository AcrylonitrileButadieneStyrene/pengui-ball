use std::sync::Arc;

use leptos::prelude::*;

stylance::import_style!(pub style, "tabs.module.css");

#[derive(Clone)]
struct TabGroup(bool, Arc<str>);

#[component]
pub fn Tabs(
    group: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] large: bool,
    children: Children,
) -> impl IntoView {
    provide_context(TabGroup(large, group.into()));

    view! { <div class=format!("{} {class}", style::tabs)>{children()}</div> }
}

#[component]
pub fn Tab(
    label: &'static str,
    #[prop(optional)] default: bool,
    children: Children,
) -> impl IntoView {
    let group = expect_context::<TabGroup>();

    view! {
        <label class:button=group.0 role="button">
            <span class="pop-out">{label}</span>
            <input type="radio" name=group.1 checked=default />
        </label>
        <div class=style::tab_content>{children()}</div>
    }
}
