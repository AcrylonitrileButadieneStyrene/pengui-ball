use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

pub struct State {
    pub inner: RwSignal<HashMap<Arc<str>, RwSignal<Player>>>,
    pub count: RwSignal<Option<u32>>,
    pub uuids: RwSignal<HashMap<i32, Arc<str>>>,
}

impl std::ops::Deref for State {
    type Target = RwSignal<HashMap<Arc<str>, RwSignal<Player>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            inner: RwSignal::new(HashMap::new()),
            count: RwSignal::new(None),
            uuids: RwSignal::new(HashMap::default()),
        }
    }
}

impl State {
    pub fn get_or_init(&self, uuid: &Arc<str>) -> RwSignal<Player> {
        self.with_untracked(|players| players.get(uuid).copied())
            .unwrap_or_else(|| {
                let signal = RwSignal::new(crate::state::Player::default());
                self.update(|players| assert!(players.insert(uuid.clone(), signal).is_none()));
                signal
            })
    }
}

#[derive(Clone, Default)]
pub struct Player {
    pub name: Option<Arc<str>>,
    pub system: Option<Arc<str>>,
    pub rank: u32,
    pub account: bool,
    pub badge: Option<Arc<str>>,
    pub medals: [u32; 5],
}
