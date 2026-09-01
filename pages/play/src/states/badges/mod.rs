use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

mod language;
mod metadata;

pub use language::BadgeTranslation;
pub use metadata::BadgeMetadata;

type RawMetadata = Box<[Arc<BadgeMetadata>]>;
type RawLanguage = HashMap<Arc<str>, HashMap<Arc<str>, Arc<BadgeTranslation>>>;
type RawCategory = HashMap<Arc<str>, HashMap<Arc<str>, Arc<str>>>;

pub struct Badges {
    pub badge_by_id: Memo<BadgeById>,
    pub badge_by_category: Memo<BadgeByCategory>,
    pub badge_to_translation: Memo<BadgeToTranslation>,
    pub category_to_translation: Memo<CategoryToTranslation>,

    metadata_resource: LocalResource<RawMetadata>,
    language_resource: LocalResource<RawLanguage>,
    category_resource: LocalResource<RawCategory>,
}

impl Badges {
    pub fn new(game: &str) -> Self {
        let metadata_resource = metadata_resource(game);
        let language_resource = language_resource(game, "en");
        let category_resource = category_resource(game, "en");
        Self {
            badge_by_id: Memo::new(badge_by_id(metadata_resource)),
            badge_by_category: Memo::new(by_game_category(metadata_resource)),
            badge_to_translation: Memo::new(badge_to_language(language_resource)),
            category_to_translation: Memo::new(category_to_translation(category_resource)),
            metadata_resource,
            language_resource,
            category_resource,
        }
    }

    pub fn refetch(&self) {
        self.metadata_resource.refetch();
        self.language_resource.refetch();
        self.category_resource.refetch();
    }
}

fn metadata_resource(game: &str) -> LocalResource<RawMetadata> {
    let uri: std::sync::Arc<str> =
        format!("https://api.ynoproject.net/{game}/api/badge?command=list").into();
    LocalResource::new(move || {
        let uri = uri.clone();
        async move {
            let Ok(response) = gloo_net::http::Request::get(&uri)
                .credentials(leptos::web_sys::RequestCredentials::Include)
                .send()
                .await
            else {
                return Box::default();
            };

            response.json().await.unwrap_or_default()
        }
    })
}

type BadgeById = HashMap<Arc<str>, Arc<BadgeMetadata>>;
fn badge_by_id(resource: LocalResource<RawMetadata>) -> impl Fn(Option<&BadgeById>) -> BadgeById {
    move |_| {
        resource
            .read()
            .as_ref()
            .map(|badges| {
                badges
                    .iter()
                    .map(|badge| (badge.badge_id.clone(), badge.clone()))
                    .collect::<HashMap<Arc<str>, Arc<BadgeMetadata>>>()
            })
            .unwrap_or_default()
    }
}

fn language_resource(game: &str, language: &str) -> LocalResource<RawLanguage> {
    let uri: std::sync::Arc<str> =
        format!("https://ynoproject.net/{game}/lang/badge/{language}.json").into();
    LocalResource::new(move || {
        let uri = uri.clone();
        async move {
            let Ok(response) = gloo_net::http::Request::get(&uri)
                .credentials(leptos::web_sys::RequestCredentials::Include)
                .send()
                .await
            else {
                return HashMap::default();
            };

            response.json().await.unwrap_or_default()
        }
    })
}

type BadgeToTranslation = HashMap<Arc<str>, Arc<BadgeTranslation>>;
fn badge_to_language(
    resource: LocalResource<RawLanguage>,
) -> impl Fn(Option<&BadgeToTranslation>) -> BadgeToTranslation {
    move |_| {
        resource
            .read()
            .as_ref()
            .map(|badges| {
                badges
                    .iter()
                    .flat_map(|(_, badges)| badges)
                    .map(|(badge_id, badge)| (badge_id.clone(), badge.clone()))
                    .collect::<HashMap<Arc<str>, Arc<BadgeTranslation>>>()
            })
            .unwrap_or_default()
    }
}

type BadgeByCategory =
    HashMap<Arc<str>, HashMap<Option<Arc<str>>, Arc<[Arc<metadata::BadgeMetadata>]>>>;
fn by_game_category(
    resource: LocalResource<RawMetadata>,
) -> impl Fn(Option<&BadgeByCategory>) -> BadgeByCategory {
    move |_| {
        resource
            .read()
            .as_ref()
            .map(|badges| {
                badges.iter().fold(HashMap::new(), |mut map, badge| {
                    map.entry(badge.game.clone())
                        .or_insert_with(Vec::new)
                        .push(badge.clone());
                    map
                })
            })
            .map(|badges| {
                badges
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            key,
                            value
                                .into_iter()
                                .fold(HashMap::new(), |mut map, badge| {
                                    map.entry(if badge.group.is_empty() {
                                        None
                                    } else {
                                        Some(badge.group.clone())
                                    })
                                    .or_insert_with(Vec::new)
                                    .push(badge.clone());
                                    map
                                })
                                .into_iter()
                                .map(|(key, mut value)| {
                                    value.sort();
                                    (key, value.into())
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

fn category_resource(game: &str, language: &str) -> LocalResource<RawCategory> {
    let uri: std::sync::Arc<str> =
        format!("https://ynoproject.net/{game}/lang/badge/groups/{language}.json").into();
    LocalResource::new(move || {
        let uri = uri.clone();
        async move {
            let Ok(response) = gloo_net::http::Request::get(&uri)
                .credentials(leptos::web_sys::RequestCredentials::Include)
                .send()
                .await
            else {
                return HashMap::default();
            };

            response.json().await.unwrap_or_default()
        }
    })
}

type CategoryToTranslation = RawCategory;
fn category_to_translation(
    resource: LocalResource<RawCategory>,
) -> impl Fn(Option<&CategoryToTranslation>) -> CategoryToTranslation {
    move |_| resource.get().unwrap_or_default()
}
