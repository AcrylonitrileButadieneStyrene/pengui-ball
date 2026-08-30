use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

mod badge;

pub struct Badges {
    pub all: Memo<AllBadges>,
    pub games: Memo<GameBadges>,

    resource: LocalResource<Option<Box<[Arc<badge::Badge>]>>>,
}

impl Badges {
    pub fn new(game: &str) -> Self {
        let resource = badges_resource(game);
        Self {
            all: Memo::new(all_badges(resource)),
            games: Memo::new(game_badges(resource)),
            resource,
        }
    }

    pub fn refetch(&self) {
        self.resource.refetch();
    }
}

fn badges_resource(game: &str) -> LocalResource<Option<Box<[Arc<badge::Badge>]>>> {
    let uri: std::sync::Arc<str> =
        format!("https://api.ynoproject.net/{game}/api/badge?command=list").into();
    LocalResource::new(move || {
        let uri = uri.clone();
        async move {
            gloo_net::http::Request::get(&uri)
                .credentials(leptos::web_sys::RequestCredentials::Include)
                .send()
                .await
                .ok()?
                .json::<Box<[Arc<badge::Badge>]>>()
                .await
                .ok()
        }
    })
}

type AllBadges = HashMap<Arc<str>, Arc<badge::Badge>>;
fn all_badges(
    resource: LocalResource<Option<Box<[Arc<badge::Badge>]>>>,
) -> impl Fn(Option<&AllBadges>) -> AllBadges {
    move |_| {
        resource
            .read()
            .as_ref()
            .flatten()
            .map(|badges| {
                badges
                    .iter()
                    .map(|badge| (badge.badge_id.clone(), badge.clone()))
                    .collect::<HashMap<Arc<str>, Arc<badge::Badge>>>()
            })
            .unwrap_or_default()
    }
}

type GameBadges = HashMap<Arc<str>, HashMap<Option<Arc<str>>, Arc<[Arc<badge::Badge>]>>>;
fn game_badges(
    resource: LocalResource<Option<Box<[Arc<badge::Badge>]>>>,
) -> impl Fn(Option<&GameBadges>) -> GameBadges {
    move |_| {
        resource
            .read()
            .as_ref()
            .flatten()
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
