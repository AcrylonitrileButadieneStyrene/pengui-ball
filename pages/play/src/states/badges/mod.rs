use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

mod language;
mod metadata;

pub use language::BadgeTranslation;
pub use metadata::BadgeMetadata;

type RawMetadata = Box<[Arc<BadgeMetadata>]>;
type RawLanguage = HashMap<Arc<str>, HashMap<Arc<str>, Arc<BadgeTranslation>>>;

pub struct Badges {
    pub metadata: Memo<BadgesMetadata>,
    pub by_group: Memo<BadgesCategory>,
    pub language: Memo<BadgesLanguage>,

    metadata_resource: LocalResource<RawMetadata>,
    language_resource: LocalResource<RawLanguage>,
}

impl Badges {
    pub fn new(game: &str) -> Self {
        let metadata_resource = metadata_resource(game);
        let language_resource = language_resource(game, "en");
        Self {
            metadata: Memo::new(metadata(metadata_resource)),
            by_group: Memo::new(category(metadata_resource)),
            language: Memo::new(language(language_resource)),
            metadata_resource,
            language_resource,
        }
    }

    pub fn refetch(&self) {
        self.metadata_resource.refetch();
        self.language_resource.refetch();
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

type BadgesMetadata = HashMap<Arc<str>, Arc<BadgeMetadata>>;
fn metadata(
    resource: LocalResource<RawMetadata>,
) -> impl Fn(Option<&BadgesMetadata>) -> BadgesMetadata {
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

type BadgesLanguage = HashMap<Arc<str>, Arc<BadgeTranslation>>;
fn language(
    resource: LocalResource<RawLanguage>,
) -> impl Fn(Option<&BadgesLanguage>) -> BadgesLanguage {
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

type BadgesCategory =
    HashMap<Arc<str>, HashMap<Option<Arc<str>>, Arc<[Arc<metadata::BadgeMetadata>]>>>;
fn category(
    resource: LocalResource<RawMetadata>,
) -> impl Fn(Option<&BadgesCategory>) -> BadgesCategory {
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
