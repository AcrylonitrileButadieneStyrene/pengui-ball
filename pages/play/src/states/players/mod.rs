use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

pub mod friend;
pub mod player;

pub struct Players {
    pub by_uuid: RwSignal<HashMap<Arc<str>, RwSignal<player::Player>>>,
    pub uuids: RwSignal<HashMap<i32, Arc<str>>>,
    pub count: RwSignal<Option<u32>>,

    pub friends: RwSignal<HashMap<Arc<str>, RwSignal<friend::Friend>>>,
}

impl Players {
    pub fn new() -> Self {
        Self {
            by_uuid: RwSignal::new(HashMap::new()),
            uuids: RwSignal::new(HashMap::default()),
            count: RwSignal::new(None),
            friends: RwSignal::new(HashMap::new()),
        }
    }

    pub fn get_or_init(&self, uuid: &Arc<str>) -> RwSignal<player::Player> {
        self.by_uuid
            .with_untracked(|players| players.get(uuid).copied())
            .unwrap_or_else(|| {
                let signal = RwSignal::new(player::Player::default());
                self.by_uuid
                    .update(|players| assert!(players.insert(uuid.clone(), signal).is_none()));
                signal
            })
    }
}
