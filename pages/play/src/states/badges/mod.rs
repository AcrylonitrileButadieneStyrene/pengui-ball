use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

mod badge;

pub struct Badges {
    pub games: ReadSignal<HashMap<Arc<str>, Arc<[Arc<badge::Badge>]>>>,

    resource: LocalResource<Option<Box<[Arc<badge::Badge>]>>>,
}

impl Badges {
    pub fn new(game: &str) -> Self {
        let (games, set_games) = signal(HashMap::default());

        let resource = badges_resource(game, set_games);
        Self { games, resource }
    }

    pub fn refetch(&self) {
        self.resource.refetch();
    }
}

fn badges_resource(
    game: &str,
    set_games: WriteSignal<HashMap<Arc<str>, Arc<[Arc<badge::Badge>]>>>,
) -> LocalResource<Option<Box<[Arc<badge::Badge>]>>> {
    let uri: std::sync::Arc<str> =
        format!("https://api.ynoproject.net/{game}/api/badge?command=list").into();
    let resource = LocalResource::new(move || {
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
    });

    Effect::new(move || {
        let Some(badges) = resource.get() else {
            return;
        };

        set_games(
            badges
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
                        .map(|(key, mut value)| {
                            value.sort();
                            (key, value.into())
                        })
                        .collect()
                })
                .unwrap_or_default(),
        );
    });

    resource
}
