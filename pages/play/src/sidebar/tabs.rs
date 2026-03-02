use leptos::prelude::*;

stylance::import_style!(pub style, "tabs.module.css");

#[component]
pub fn Tab(label: String, #[prop(optional)] default: bool, children: Children) -> impl IntoView {
    view! {
        <label class=style::label class:button=true>
            <span class="pop-out">{label}</span>
            <input type="radio" name="selected-sidebar-tab" checked=default/>
        </label>
        <div class=style::tab_content>{children()}</div>
    }
}
