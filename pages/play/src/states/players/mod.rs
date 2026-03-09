use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;
use reactive_stores::Store;

pub mod friend;
pub mod player;

#[derive(Default)]
pub struct Players {
    pub all: RwSignal<HashMap<Arc<str>, Store<player::Player>>>,
    pub count: RwSignal<Option<u32>>,
    pub local: Store<player::Player>,
    pub in_map: RwSignal<HashMap<u16, Store<player::Player>>>,
    pub friends: RwSignal<Vec<friend::Friend>>,
}

impl Players {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_init(&self, uuid: &Arc<str>) -> Store<player::Player> {
        self.all
            .with_untracked(|players| players.get(uuid).copied())
            .unwrap_or_else(|| {
                let signal = Store::new(player::Player::default());
                self.all
                    .update(|players| assert!(players.insert(uuid.clone(), signal).is_none()));
                signal
            })
    }

    pub fn get_by_id(&self, id: i32) -> Option<Store<player::Player>> {
        if id == -1 {
            Some(self.local)
        } else {
            self.in_map.read_untracked().get(&(id as u16)).cloned()
        }
    }
}
