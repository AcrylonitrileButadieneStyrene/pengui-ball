use std::sync::Arc;

use leptos::prelude::*;

pub mod maps;

pub struct Locations {
    pub game: Arc<str>,
    // pub resolver: Arc<locations::Resolver>,
    pub current: RwSignal<Option<locations::Location>>,
    pub current_resolved: Signal<Option<locations::LocationResolved>>,
    pub current_maps: LocalResource<Vec<maps::Map>>,
}

impl Locations {
    pub fn new(game: Arc<str>) -> Self {
        let resolver = Arc::new(locations::Resolver::new_prefetch(game.clone()));
        provide_context(resolver.clone());

        let current = RwSignal::new(None);
        let current_resolved =
            Signal::derive(move || current.get().map(|location| resolver.resolve(&location)));

        Self {
            game,
            current,
            current_resolved,
            current_maps: maps::resource(current_resolved),
        }
    }
}
