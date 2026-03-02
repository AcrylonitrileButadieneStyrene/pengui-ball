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
        <div class=style::panes data-selected=move || selected().get_str("Name")>
            {children()}
        </div>
    }
}

#[component]
fn TabButton(
    selected: ReadSignal<SelectedTab>,
    set_selected: WriteSignal<SelectedTab>,
    target: SelectedTab,
) -> impl IntoView {
    target.get_str("Name").map(move |name| {
        view! {
            <label class="button">
                <span class="pop-out">{name}</span>
                <input
                    type="radio"
                    name="selected-sidebar-tab"
                    prop:checked=move || selected() == target
                    on:change=move |event| {
                        if event_target_checked(&event) {
                            set_selected(target);
                        }
                    }
                />
            </label>
        }
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
