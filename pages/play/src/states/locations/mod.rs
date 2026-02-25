use std::sync::Arc;

use leptos::prelude::*;

mod location;
mod resolver;

pub use location::Location;
pub use resolver::LocationResolved;

pub struct Locations {
    pub game: Arc<str>,
    pub resolver: Arc<resolver::LocationResolver>,
    pub current: RwSignal<Option<Location>>,
    pub current_resolved: Signal<Option<resolver::LocationResolved>>,
}

impl Locations {
    pub fn new(game: Arc<str>) -> Self {
        let resolver = Arc::new(resolver::LocationResolver::new_prefetch(game.clone()));
        let current = RwSignal::new(None);

        Locations {
            game,
            resolver: resolver.clone(),
            current,
            current_resolved: Signal::derive(move || {
                current.get().map(|location| resolver.resolve(&location))
            }),
        }
    }
}
