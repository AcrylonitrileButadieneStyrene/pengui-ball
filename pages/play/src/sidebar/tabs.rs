use leptos::prelude::*;
use strum::{EnumProperty, VariantArray};

stylance::import_style!(pub style, "tabs.module.css");

#[island]
pub fn Tabs(children: Children) -> impl IntoView {
    let (selected, set_selected) = signal(SelectedTab::default());
    provide_context(selected);

    let tabs = SelectedTab::VARIANTS
        .iter()
        .map(|variant| view! { <TabButton selected set_selected target=*variant /> })
        .collect::<Vec<_>>();

    view! {
        <div class=style::tabs>{tabs}</div>
        {children()}
    }
}

#[component]
fn TabButton(
    selected: ReadSignal<SelectedTab>,
    set_selected: WriteSignal<SelectedTab>,
    target: SelectedTab,
) -> impl IntoView {
    target.get_str("Name").map(|name| {
        view! { <label role="button">{name} <input type="radio" name="selected-sidebar-tab" /></label> }
    })
}

#[derive(Clone, Copy, Default, PartialEq, Eq, strum::EnumProperty, strum::VariantArray)]
pub enum SelectedTab {
    #[default]
    #[strum(props(Name = "Chat"))]
    Chat,
    #[strum(props(Name = "Players"))]
    Players,
    #[strum(props(Name = "Parties"))]
    Parties,
}
