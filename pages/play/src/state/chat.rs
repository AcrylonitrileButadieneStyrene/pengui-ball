use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use leptos::prelude::*;

use crate::state::Message;

pub struct ChatState {
    pub messages: ReadSignal<indexmap::IndexMap<Arc<str>, Message>>,

    pub map: ChatChannel,
    pub party: ChatChannel,
    pub global: ChatChannel,
}

impl Default for ChatState {
    fn default() -> Self {
        let (messages, set_messages) = signal(indexmap::IndexMap::new());

        Self {
            messages,
            map: ChatChannel::new(set_messages, 150, false),
            party: ChatChannel::new(set_messages, 150, false),
            global: ChatChannel::new(set_messages, 150, false),
        }
    }
}

pub struct ChatChannel {
    pub capacity: RwSignal<usize>,
    pub tracker: Mutex<VecDeque<Arc<str>>>,
    pub filter: RwSignal<bool>,
    messages: WriteSignal<indexmap::IndexMap<Arc<str>, Message>>,
}

impl ChatChannel {
    pub fn new(
        messages: WriteSignal<indexmap::IndexMap<Arc<str>, Message>>,
        capacity: usize,
        filtered: bool,
    ) -> Self {
        Self {
            capacity: RwSignal::new(capacity),
            tracker: Mutex::new(VecDeque::with_capacity(capacity)),
            filter: RwSignal::new(filtered),
            messages,
        }
    }

    pub fn add(&self, mut message: Message) {
        message.filtered = Some(self.filter.read_only());

        let mut buffer = self.tracker.lock().unwrap();
        let removed = if buffer.len() + 1 > self.capacity.get_untracked() {
            buffer.pop_back()
        } else {
            None
        };

        let id: Arc<str> = message.id.clone().into();
        buffer.push_front(id.clone());

        self.messages.update(|messages| {
            if let Some(id) = removed {
                messages.shift_remove(&id);
            }

            messages.insert(id, message);
        });
    }
}
