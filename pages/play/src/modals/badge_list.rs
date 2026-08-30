use std::sync::Arc;

use leptos::prelude::*;

stylance::import_style!(pub style, "badge_list.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::BadgeList>
            <h1>Badges</h1>
            <Provider>
                <GameSelector>
                    <GameTab id="all" label="All" default=true />
                </GameSelector>

                <div style:height="10000px">
                    <Inner/>
                </div>
            </Provider>
        </super::Modal>
    }
}

#[derive(Clone, Copy)]
struct SelectedGame(pub RwSignal<Option<Arc<str>>>);

#[island]
fn Provider(children: Children) -> impl IntoView {
    provide_context(SelectedGame(RwSignal::new(None)));

    children()
}

#[island]
fn GameSelector(children: Children) -> impl IntoView {
    let state = crate::state();
    let selected_game = expect_context::<SelectedGame>();

    let on_change = move |event| {
        let game = event_target_value(&event);
        selected_game.0.set(if game == "all" {
            None
        } else {
            Some(game.into())
        });
    };

    view! {
        <div class=style::games on:change=on_change>
            {children()}
            <For each=state.badges.by_group key=|(game, _)| game.clone() let((game, _))>
                <GameTab id=game.clone() label=game />
            </For>
        </div>
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
