use std::{
    collections::HashMap,
    sync::{Arc, nonpoison::RwLock},
};

use leptos::prelude::*;

mod classic;
mod explorer;

pub struct Locations {
    classic: classic::Container,
    explorer: explorer::Container,
}

impl Locations {
    pub fn new_prefetch(game: Arc<str>) -> Self {
        let mut map = HashMap::new();
        let resource = classic::fetch(&game);
        map.insert(game, resource);
        Self {
            classic: RwLock::new(map),
            explorer: RwSignal::new(HashMap::new()),
        }
    }

    pub fn get(&self, game: &str) -> LocalResource<Result<classic::LocationData, gloo_net::Error>> {
        self.classic.read().get(game).map_or_else(
            || {
                let resource = classic::fetch(game);
                self.classic.write().insert(Arc::from(game), resource);
                resource
            },
            |resource| *resource,
        )
    }

    pub fn resolve(&self, location: &super::super::game::Location) -> Option<ResolvedLocation> {
        let resolved = self
            .get(&location.game)
            .read()
            .as_ref()
            .map(Result::as_ref)
            .and_then(Result::ok)
            .and_then(|locations| {
                locations
                    .maps
                    .get(&*format!("{:>04}", location.map))
                    .zip(Some(locations.root.clone()))
            })
            .and_then(|(item, wiki)| {
                classic::resolve(item, location.previous, location.x, location.y).zip(Some(wiki))
            })
            .map(|((name, article), wiki)| ResolvedLocation::Single {
                wiki: wiki
                    .as_ref()
                    .map(|root| root.to_string() + article.as_ref().unwrap_or(&name)),
                name,
            });

        if resolved.is_none() && &*location.game == "2kki" {
            let value = self
                .explorer
                .with_untracked(|explorer| {
                    explorer.get(&(location.map, location.previous)).cloned()
                })
                .map_or_else(
                    || {
                        explorer::fetch(self.explorer, location.map, location.previous);
                        None
                    },
                    |entry| entry.lock().clone().map(ResolvedLocation::Multiple),
                );

            if value.is_none() {
                self.explorer.track();
            }

            value
        } else {
            resolved
        }
    }
}

pub enum ResolvedLocation {
    Single {
        name: Arc<str>,
        wiki: Option<String>,
    },
    Multiple(Arc<[explorer::Location]>),
}
