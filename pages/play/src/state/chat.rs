use std::collections::VecDeque;

use leptos::prelude::*;

use crate::state::Message;

pub struct ChatState {
    pub map_cap: RwSignal<usize>,
    pub party_cap: RwSignal<usize>,
    pub global_cap: RwSignal<usize>,

    pub map: RwSignal<VecDeque<Message>>,
    pub party: RwSignal<VecDeque<Message>>,
    pub global: RwSignal<VecDeque<Message>>,
}

impl Default for ChatState {
    fn default() -> Self {
        Self {
            map_cap: RwSignal::new(150),
            party_cap: RwSignal::new(150),
            global_cap: RwSignal::new(150),
            map: RwSignal::new(VecDeque::with_capacity(150)),
            party: RwSignal::new(VecDeque::with_capacity(150)),
            global: RwSignal::new(VecDeque::with_capacity(150)),
        }
    }
}

impl ChatState {
    pub fn add_map(&self, message: Message) {
        self.map.update(|chat| {
            if chat.len() + 1 > self.map_cap.get_untracked() {
                chat.pop_back();
            }
            chat.push_front(message);
        });
    }

    pub fn add_party(&self, message: Message) {
        self.party.update(|chat| {
            if chat.len() + 1 > self.party_cap.get_untracked() {
                chat.pop_back();
            }
            chat.push_front(message);
        });
    }

    pub fn add_global(&self, message: Message) {
        self.global.update(|chat| {
            if chat.len() + 1 > self.global_cap.get_untracked() {
                chat.pop_back();
            }
            chat.push_front(message);
        });
    }
}
