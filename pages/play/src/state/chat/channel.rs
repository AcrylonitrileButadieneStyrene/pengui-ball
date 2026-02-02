use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use leptos::prelude::*;

use crate::state::MessageData;

use super::Message;

pub struct ChatChannel {
    pub capacity: RwSignal<usize>,
    pub tracker: Mutex<VecDeque<Arc<str>>>,
    pub filter: RwSignal<bool>,
    pub local: Mutex<Vec<(Arc<str>, Arc<str>)>>,
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
            local: Mutex::new(Vec::new()),
            messages,
        }
    }

    pub fn add(&self, mut message: Message) {
        message.filtered = Some(self.filter.read_only());

        if let MessageData::Local { ref text } = message.data {
            self.local
                .lock()
                .unwrap()
                .push((message.id.clone(), text.clone()));
            self.messages.update(|messages| {
                messages.insert(message.id.clone(), message);
            });
        } else {
            let mut buffer = self.tracker.lock().unwrap();
            let removed_overflow = if buffer.len() + 1 > self.capacity.get_untracked() {
                buffer.pop_back()
            } else {
                None
            };

            buffer.push_front(message.id.clone());
            drop(buffer);

            let mut local = self.local.lock().unwrap();
            let removed_duplicate = local
                .iter()
                .position(|(_, text)| message.text().eq(&Some(text)))
                .map(|index| local.swap_remove(index).0);
            drop(local);

            self.messages.update(|messages| {
                if let Some(id) = removed_overflow {
                    messages.shift_remove(&id).unwrap();
                }

                if let Some(id) = removed_duplicate {
                    messages.shift_remove(&id).unwrap();
                }

                messages.insert(message.id.clone(), message);
            });
        }
    }
}
