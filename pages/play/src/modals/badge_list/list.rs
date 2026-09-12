use std::sync::Arc;

use indexmap::IndexMap;
use leptos::prelude::*;

#[component]
pub fn List(
    badges: super::badges::Badges,
    selected_game: ReadSignal<Option<Arc<str>>>,
    selected_group: ReadSignal<Option<Arc<str>>>,
) -> impl IntoView {
    let view = move || {
        badges
            .get()
            .into_iter()
            .map(|(game_id, super::badges::BadgeGame { badges, .. })| {
                view! { <Game badges game_id selected_game selected_group /> }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class=super::style::scroller>
            <div class=super::style::container>{view}</div>
        </div>
    }
}

#[component]
fn Game(
    badges: Arc<IndexMap<Option<Arc<str>>, Arc<[Arc<super::badges::Badge>]>>>,
    game_id: Arc<str>,
    selected_game: ReadSignal<Option<Arc<str>>>,
    selected_group: ReadSignal<Option<Arc<str>>>,
) -> impl IntoView {
    let visible = move || selected_game().is_none_or(|selected| game_id == selected);

    let view = badges
        .iter()
        .map(|(category_id, badges)| {
            view! { <Category badges=badges.clone() category_id=category_id.clone() selected_group /> }
        })
        .collect::<Vec<_>>();

    view! {
        <div prop:style=move || {
            if visible() { "display:children;" } else { "display:none;" }
        }>{view}</div>
    }
}

#[component]
fn Category(
    badges: Arc<[Arc<super::badges::Badge>]>,
    category_id: Option<Arc<str>>,
    selected_group: ReadSignal<Option<Arc<str>>>,
) -> impl IntoView {
    let visible = move || {
        // note: the `is_some_and` call makes it so uncategorized badges
        //   are not visible unless you are on the all category which is
        //   the behavior of forest-orb. perhaps this should be changed?
        selected_group().is_none_or(|selected| {
            category_id
                .as_ref()
                .is_some_and(|category| *category == selected)
        })
    };

    let view = badges
        .iter()
        .map(|badge| {
            view! { <Badge meta=badge.metadata.clone() lang=badge.language.clone() /> }
        })
        .collect::<Vec<_>>();

    view! {
        <div prop:style=move || {
            if visible() { "display:children;" } else { "display:none;" }
        }>{view}</div>
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

    view! { <img class=super::style::badge src=src loading="lazy" /> }
}
