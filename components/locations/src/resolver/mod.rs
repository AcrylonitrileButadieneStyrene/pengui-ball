use std::{
    collections::HashMap,
    sync::{Arc, nonpoison::Mutex},
};

use leptos::prelude::*;

use super::Location;

pub mod classic;
pub mod explorer;

pub struct Resolver {
    owner: Owner,
    classic: classic::Container,
    explorer: explorer::Container,
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
            owner: Owner::current().unwrap(),
            classic: Mutex::new(HashMap::new()),
            explorer: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Resolver {
    pub fn new_prefetch(game: Arc<str>) -> Self {
        let new = Self::default();
        let resource = classic::fetch(&game);
        new.classic.lock().insert(game, resource);
        new
    }

    pub fn get_or_init(
        &self,
        game: &str,
    ) -> LocalResource<Result<classic::LocationData, gloo_net::Error>> {
        let resource = self.classic.lock().get(game).copied();

        resource.unwrap_or_else(|| {
            let resource = classic::fetch_with_owner(game, &self.owner);
            self.classic.lock().insert(Arc::from(game), resource);
            resource
        })
    }

    pub fn resolve(&self, location: &Location) -> LocationResolved {
        let resolved = self.resolve_wiki(location);
        let ret =
            if matches!(resolved, LocationResolved::Unknown { .. }) && &*location.game == "2kki" {
                self.resolve_2kki(location)
            } else {
                resolved
            };
        leptos::logging::log!("{ret:?}");
        ret
    }

    fn resolve_wiki(&self, location: &Location) -> LocationResolved {
        let Location {
            ref game,
            map,
            previous,
            x,
            y,
        } = *location;

        let Some(Ok(ref locations)) = *self.get_or_init(game).read() else {
            // waiting for the wiki data to download
            return LocationResolved::Pending;
        };

        let unknown = LocationResolved::Unknown { map, x, y };
        locations
            .maps
            .get(&*format!("{map:>04}"))
            .map_or(unknown.clone(), move |map| {
                let locations = classic::resolve(map, previous, x, y)
                    .iter()
                    .map(|(name, article)| classic::Location {
                        wiki: locations.root.as_ref().map(|root| {
                            Arc::from(root.to_string() + article.as_ref().unwrap_or(&name))
                        }),
                        name: name.clone(),
                    })
                    .collect::<Vec<_>>();

                if locations.is_empty() {
                    unknown
                } else {
                    LocationResolved::Classic(locations.into())
                }
            })
    }

    fn resolve_2kki(&self, location: &Location) -> LocationResolved {
        let Location {
            map,
            x,
            y,
            previous,
            ..
        } = *location;

        let value = self.explorer.lock().get(&(map, previous)).cloned();
        value.map_or_else(
            || {
                explorer::fetch_with_owner(self.explorer.clone(), map, previous, &self.owner);
                LocationResolved::Pending
            },
            |entry| {
                let unknown = LocationResolved::Unknown { map, x, y };
                let Some(entry) = (match entry {
                    explorer::Value::Pending(val) => val.get(),
                    explorer::Value::Resolved(val) => Some(val),
                }) else {
                    return unknown;
                };

                (*entry)
                    .as_ref()
                    .ok()
                    .cloned()
                    .map_or(unknown, LocationResolved::Explorer)
            },
        )
    }
}

#[derive(Clone, Debug)]
pub enum LocationResolved {
    Pending,
    Unknown { map: u16, x: i16, y: i16 },
    Classic(Arc<[classic::Location]>),
    Explorer(Arc<[explorer::Location]>),
}
