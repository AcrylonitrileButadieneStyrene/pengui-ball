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
    let state = crate::state();
    let (throttle, set_throttle) = signal(0);
    let throttle: Signal<usize> = leptos_use::signal_throttled_with_options(
        throttle,
        60_000.,
        leptos_use::ThrottleOptions {
            trailing: false,
            leading: true,
        },
    );

    Effect::new(move || {
        if state.modal.get() == Some(super::Modals::BadgeList) {
            set_throttle.update(|x| {
                *x = x.checked_add(1).unwrap_or(1);
            });
        }
    });

    Effect::new(move || {
        if throttle.get() != 0 {
            state.badges.refetch();
        }
    });

    let selected_game = RwSignal::<Option<Arc<str>>>::default();
    let selected_group = RwSignal::<Option<Arc<str>>>::default();

    view! {
        <GameSelector selected_game />
        <GroupSelector selected_game=selected_game.read_only() selected_group />
        <List selected_game=selected_game.read_only() selected_group=selected_group.read_only() />
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
fn GroupSelector(
    selected_game: ReadSignal<Option<Arc<str>>>,
    selected_group: RwSignal<Option<Arc<str>>>,
) -> impl IntoView {
    let state = crate::state();
    let all_group = NodeRef::<leptos::html::Input>::new();

    Effect::new(move || {
        selected_game.track();
        if let Some(all) = all_group.get() {
            all.set_checked(true);
        }
    });

    let categories = move || {
        selected_game
            .get()
            .and_then(|game| {
                let by_game = state.badges.category_to_translation.read();
                by_game.get(&game).cloned()
            })
            .unwrap_or_default()
    };

    let on_change = move |event| {
        let game = event_target_value(&event);
        selected_group.set(to_selection(game));
    };

    view! {
        <div class=style::games on:change=on_change>
            <SelectorTab id="all" label="All" {..} name="badge-list-group" checked=true node_ref=all_group />
            <For each=categories key=|(group, _)| group.clone() let((group, _))>
                <SelectorTab id=group.clone() label=group.clone() {..} name="badge-list-group" />
            </For>
        </div>
    }
}

fn to_selection(value: String) -> Option<Arc<str>> {
    if value == "all" {
        None::<Arc<str>>
    } else {
        Some(value.into())
    }
}

#[component(transparent)]
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
fn List(
    selected_game: ReadSignal<Option<Arc<str>>>,
    selected_group: ReadSignal<Option<Arc<str>>>,
) -> impl IntoView {
    let state = crate::state();

    let badges = Memo::new(move |_| {
        let badges = state.badges.badge_by_category.get();
        let badges = if let Some(game) = selected_game.get()
            && let Some(badges) = badges.get(&game)
        {
            if let Some(group) = selected_group.get()
                && let Some(badges) = badges.get(&Some(group))
            {
                badges.to_vec()
            } else {
                badges
                    .iter()
                    .flat_map(|(_group, badges)| badges.to_vec())
                    .collect::<Vec<_>>()
            }
        } else {
            badges
                .iter()
                .flat_map(|(_game, groups)| groups.clone())
                .flat_map(|(_group, badges)| badges.to_vec())
                .collect::<Vec<_>>()
        };

        let translations = state.badges.badge_to_translation.get();
        badges
            .into_iter()
            .map(|badge| {
                let translation = translations.get(&badge.badge_id).cloned();
                (badge, translation)
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class=style::container>
            <For each=badges key=|(badge,_)| badge.badge_id.clone() let((meta,lang))>
                <Badge meta lang />
            </For>
        </div>
    }
}

#[component]
fn Badge(
    meta: Arc<crate::states::badges::BadgeMetadata>,
    lang: Option<Arc<crate::states::badges::BadgeTranslation>>,
) -> impl IntoView {
    let src = if meta.animated {
        format!(
            "https://ynoproject.net/2kki/images/badge/{}.gif",
            meta.badge_id
        )
    } else {
        format!(
            "https://ynoproject.net/2kki/images/badge/{}.png",
            meta.badge_id
        )
    };

    view! {
        <img class=style::badge src=src loading="lazy" />
    }
}
