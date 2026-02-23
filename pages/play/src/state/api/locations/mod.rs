use std::{
    collections::HashMap,
    sync::{
        Arc,
        nonpoison::{Mutex, RwLock},
    },
};

use leptos::prelude::*;

use crate::state::game::Location;

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
            explorer: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_or_init(
        &self,
        game: &str,
    ) -> LocalResource<Result<classic::LocationData, gloo_net::Error>> {
        self.classic.read().get(game).map_or_else(
            || {
                let resource = classic::fetch(game);
                self.classic.write().insert(Arc::from(game), resource);
                resource
            },
            |resource| *resource,
        )
    }

    pub fn resolve(&self, location: &super::super::game::Location) -> ResolvedLocation {
        let resolved = self.resolve_wiki(location);
        if matches!(resolved, ResolvedLocation::Unknown) && &*location.game == "2kki" {
            self.resolve_2kki(location)
        } else {
            resolved
        }
    }

    fn resolve_wiki(&self, location: &Location) -> ResolvedLocation {
        let Location {
            ref game,
            map,
            previous,
            x,
            y,
        } = *location;

        let Some(Ok(ref locations)) = *self.get_or_init(game).read() else {
            // waiting for the wiki data to download
            return ResolvedLocation::Pending;
        };

        if let Some(map) = locations.maps.get(&*format!("{map:>04}"))
            && let Some((name, article)) = classic::resolve(map, previous, x, y)
        {
            ResolvedLocation::Single {
                wiki: locations
                    .root
                    .as_ref()
                    .map(|root| root.to_string() + article.as_ref().unwrap_or(&name)),
                name,
            }
        } else {
            ResolvedLocation::Unknown
        }
    }

    fn resolve_2kki(&self, location: &super::super::game::Location) -> ResolvedLocation {
        let value = self
            .explorer
            .lock()
            .get(&(location.map, location.previous))
            .cloned();
        value.map_or_else(
            || {
                explorer::fetch(self.explorer.clone(), location.map, location.previous);
                ResolvedLocation::Pending
            },
            |entry| {
                let Some(entry) = (match entry {
                    explorer::Value::Pending(val) => val.get(),
                    explorer::Value::Resolved(val) => Some(val),
                }) else {
                    return ResolvedLocation::Unknown;
                };

                (*entry)
                    .as_ref()
                    .ok()
                    .cloned()
                    .map_or(ResolvedLocation::Unknown, ResolvedLocation::Multiple)
            },
        )
    }
}

#[derive(Debug)]
pub enum ResolvedLocation {
    Pending,
    Unknown,
    Single {
        name: Arc<str>,
        wiki: Option<String>,
    },
    Multiple(Arc<[explorer::Location]>),
}
