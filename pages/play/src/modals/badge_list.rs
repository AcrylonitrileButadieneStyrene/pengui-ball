use std::sync::Arc;

use leptos::{attribute_interceptor::AttributeInterceptor, prelude::*};

stylance::import_style!(pub style, "badge_list.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::BadgeList>
            <h1>Badges</h1>
            <Inner />
        </super::Modal>
    }
}

#[island]
fn Inner() -> impl IntoView {
    let selected_game = RwSignal::<Option<Arc<str>>>::default();
    let selected_group = RwSignal::<Option<Arc<str>>>::default();

    view! {
        <GameSelector selected_game />
        <GroupSelector selected_game selected_group />
        <List />
    }
}

#[component]
fn GameSelector(selected_game: RwSignal<Option<Arc<str>>>) -> impl IntoView {
    let state = crate::state();

    let on_change = move |event| {
        let game = event_target_value(&event);
        selected_game.set(to_selection(game));
    };

    view! {
        <div class=style::games on:change=on_change>
            <SelectorTab id="all" label="All" {..} name="badge-list-game" checked=true />
            <For each=state.badges.badge_by_category key=|(game, _)| game.clone() let((game, _))>
                <SelectorTab id=game.clone() label=game.clone() {..} name="badge-list-game" />
            </For>
        </div>
    }
}

#[component]
fn SelectorTab(#[prop(into)] id: Arc<str>, #[prop(into)] label: Arc<str>) -> impl IntoView {
    view! {
        <AttributeInterceptor let:attrs>
            <label>
                <input type="radio" value=id.clone() autocomplete="off" {..attrs} />
                <span>{label.clone()}</span>
            </label>
        </AttributeInterceptor>
    }
}

#[component]
fn GroupSelector(
    selected_game: RwSignal<Option<Arc<str>>>,
    selected_group: RwSignal<Option<Arc<str>>>,
) -> impl IntoView {
    let state = crate::state();
    let all_group = NodeRef::new();

    let categories = move || {
        selected_game.get().and_then(|game| {
            let by_game = state.badges.category_to_translation.read();
            by_game.get(&game).cloned()
        })
    };

    view! {
        <div class=style::games>
            <label>
                <input
                    type="radio"
                    name="badge-list-group"
                    value="all"
                    autocomplete="off"
                    checked
                    node_ref=all_group
                />
                <span>All</span>
            </label>
            <For each=state.badges.badge_by_category key=|(game, _)| game.clone() let((game, _))>
                <label>
                    <input
                        type="radio"
                        name="badge-list-group"
                        value=game.clone()
                        autocomplete="off"
                    />
                    <span>{game}</span>
                </label>
            </For>
        </div>
    }
}

#[component]
fn List() -> impl IntoView {
    let state = crate::state();

    move || {
        state
            .badges
            .badge_by_category
            .get()
            .get("2kki")
            .and_then(|game| game.get(&Some("4_ch".into())).cloned())
            .unwrap_or_default()
            .into_iter()
            .map(|badge| badge.badge_id.to_string())
            .collect::<Vec<_>>()
    }
}

fn to_selection(value: String) -> Option<Arc<str>> {
    if value == "all" {
        None::<Arc<str>>
    } else {
        Some(value.into())
    }
}
