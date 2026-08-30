use std::sync::Arc;

use leptos::prelude::*;

stylance::import_style!(pub style, "badge_list.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::BadgeList>
            <h1>Badges</h1>
            <WithGameSelector>
                <div style:height="10000px">
                    <Inner/>
                </div>
            </WithGameSelector>
        </super::Modal>
    }
}

#[island]
fn WithGameSelector(children: Children) -> impl IntoView {
    let state = crate::state();

    view! {
        <div class=style::games>
            <GameTab id="all" label="All" default=true />
            <For each=state.badges.by_group key=|(game, _)| game.clone() let((game, _))>
                <GameTab id=game.clone() label=game />
            </For>
        </div>
        {children()}
    }
}

#[component]
fn GameTab(
    #[prop(into)] id: Arc<str>,
    #[prop(into)] label: Arc<str>,
    #[prop(optional)] default: bool,
) -> impl IntoView {
    view! {
        <label>
            <input type="radio" name="badge-list-game" value=id autocomplete="off" prop:checked=default />
            <span>{label}</span>
        </label>
    }
}

#[island]
fn Inner() -> impl IntoView {
    let state = crate::state();

    move || {
        state
            .badges
            .by_group
            .get()
            .get("2kki")
            .and_then(|game| game.get(&Some("4_ch".into())).cloned())
            .unwrap_or_default()
            .into_iter()
            .map(|badge| badge.badge_id.to_string())
            .collect::<Vec<_>>()
    }
}
