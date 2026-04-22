use std::sync::Arc;

use leptos::prelude::*;

pub struct Locations {
    pub game: Arc<str>,
    pub resolver: Arc<locations::Resolver>,
    pub current: RwSignal<Option<locations::Location>>,
    pub current_resolved: Signal<Option<locations::LocationResolved>>,
}

impl Locations {
    pub fn new(game: Arc<str>) -> Self {
        let resolver = Arc::new(locations::Resolver::new_prefetch(game.clone()));
        let current = RwSignal::new(None);

        Self {
            game,
            resolver: resolver.clone(),
            current,
            current_resolved: Signal::derive(move || {
                current.get().map(|location| resolver.resolve(&location))
            }),
        }
    }
}
