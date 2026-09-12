use std::{collections::HashMap, sync::Arc};

use indexmap::IndexMap;
use itertools::Itertools;
use leptos::prelude::*;

use crate::states::badges::{BadgeMetadata, BadgeToTranslation, BadgeTranslation};

pub type Badges = Memo<IndexMap<Arc<str>, BadgeGame>>;

#[derive(Debug, Clone)]
pub struct BadgeGame {
    pub name: Arc<str>,
    pub badges: Arc<IndexMap<Option<Arc<str>>, Arc<[Arc<Badge>]>>>,
}

impl PartialEq for BadgeGame {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
pub struct Badge {
    pub metadata: Arc<BadgeMetadata>,
    pub language: Option<Arc<BadgeTranslation>>,
}

pub fn get_sorted(
    state: crate::State,
    current_game: Arc<str>,
    games: Arc<IndexMap<Arc<str>, Arc<str>>>,
) -> Badges {
    Memo::new({
        let games = games.clone();
        move |_| {
            let mut badges = state.badges.badge_by_category.get();
            let translations = state.badges.badge_to_translation.get();

            let current_game_name = games
                .get(&current_game)
                .cloned()
                .unwrap_or_else(|| current_game.clone());

            let mut result = IndexMap::new();
            for (game_id, game_name) in std::iter::once((current_game.clone(), current_game_name))
                .chain(std::iter::once(("ynoproject".into(), "YNOproject".into())))
                .chain(games.iter().map(|(id, name)| (id.clone(), name.clone())))
            {
                if let Some(badges) = badges.remove(&game_id) {
                    result.insert(
                        game_id.clone(),
                        BadgeGame {
                            name: game_name,
                            badges: sort_categories(badges, &translations),
                        },
                    );
                }
            }

            result.extend(badges.drain().map(|(id, badges)| {
                (
                    id.clone(),
                    BadgeGame {
                        name: id,
                        badges: sort_categories(badges, &translations),
                    },
                )
            }));

            result
        }
    })
}

fn sort_categories(
    input: HashMap<Option<Arc<str>>, Arc<[Arc<BadgeMetadata>]>>,
    translations: &BadgeToTranslation,
) -> Arc<IndexMap<Option<Arc<str>>, Arc<[Arc<Badge>]>>> {
    Arc::new(
        input
            .into_iter()
            .sorted_by_key(|(key, _)| key.clone())
            .map(with_translations(translations))
            .collect(),
    )
}

fn with_translations(
    translations: &BadgeToTranslation,
) -> impl Fn((Option<Arc<str>>, Arc<[Arc<BadgeMetadata>]>)) -> (Option<Arc<str>>, Arc<[Arc<Badge>]>)
{
    |(key, badges)| {
        (
            key,
            badges
                .into_iter()
                .map(|badge| get_translation(badge.clone(), translations))
                .collect(),
        )
    }
}

fn get_translation(badge: Arc<BadgeMetadata>, translations: &BadgeToTranslation) -> Arc<Badge> {
    Arc::new(Badge {
        language: translations.get(&badge.badge_id).cloned(),
        metadata: badge,
    })
}
