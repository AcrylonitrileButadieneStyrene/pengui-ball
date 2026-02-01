use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use leptos::prelude::*;

use super::Message;

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

        buffer.push_front(message.id.clone());
        drop(buffer);

        self.messages.update(|messages| {
            if let Some(id) = removed {
                messages.shift_remove(&id);
            }

            messages.insert(message.id.clone(), message);
        });
    }
}
