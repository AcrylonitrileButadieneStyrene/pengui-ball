use std::sync::Arc;

use leptos::prelude::*;

pub struct State {
    pub id: Arc<str>,
    pub location: RwSignal<Option<Location>>,
}

impl State {
    pub fn new(id: Arc<str>) -> Self {
        Self {
            id,
            location: RwSignal::new(None),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    pub map: u16,
    pub previous: Option<u16>,
    pub x: u16,
    pub y: u16,
}
