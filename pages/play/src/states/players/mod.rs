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

    pub fn get_or_init(&self, uuid: &Arc<str>, is_local: bool) -> Store<player::Player> {
        let existing = self.all.read_untracked().get(uuid).copied();
        let updated = if let Some(existing) = existing {
            if is_local && existing != self.local {
                self.local.set(existing.get_untracked());
                self.local
            } else {
                return existing;
            }
        } else {
            if is_local {
                self.local
            } else {
                Store::new(player::Player::default())
            }
        };

        self.all.update(|players| {
            players.insert(uuid.clone(), updated);
        });

        updated
    }

    pub fn get_by_id(&self, id: i32) -> Option<Store<player::Player>> {
        if id == -1 {
            Some(self.local)
        } else {
            self.in_map.read_untracked().get(&(id as u16)).cloned()
        }
    }
}
