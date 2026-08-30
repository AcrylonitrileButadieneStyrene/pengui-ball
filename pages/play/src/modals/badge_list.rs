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

// struct SelectedGame(std::sync::Arc<str>);

#[island]
fn WithGameSelector(children: Children) -> impl IntoView {
    let state = crate::state();

    view! {
        <div class=style::games>
            <label>
                <input type="radio" name="badge-list-game" value="all" autocomplete="off" checked />
                <span>All</span>
            </label>
            <For each=state.badges.by_group key=|(game, _)| game.clone() let((game, _))>
                <label>
                    <input type="radio" name="badge-list-game" value=game.clone() autocomplete="off" />
                    <span>{game}</span>
                </label>
            </For>
        </div>
        {children()}
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
