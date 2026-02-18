use std::{
    collections::VecDeque,
    sync::{Arc, nonpoison::Mutex},
};

use leptos::prelude::*;

use super::Message;

pub struct ChatChannel {
    pub capacity: RwSignal<usize>,
    /// List of IDs corresponding to messages in the primary `messages` list
    pub tracker: Mutex<VecDeque<(Arc<str>, Arc<str>)>>,
    /// If true then this channel is not shown to the user
    pub filter: RwSignal<bool>,
    /// Shared message list, all channels interact with it.
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

    pub fn add(&self, message: Message) {
        let mut buffer = self.tracker.lock();
        let removed_overflow = if buffer.len() + 1 > self.capacity.get_untracked() {
            buffer.pop_back()
        } else {
            None
        };

        buffer.push_front((message.id.clone(), message.text().clone()));
        drop(buffer);

        self.messages.update(|messages| {
            if let Some((id, _)) = removed_overflow {
                messages.shift_remove(&id).unwrap();
            }

            messages.insert(message.id.clone(), message);
        });
    }

    pub fn remove(&self, message: &Message) {
        let message_text = message.text();
        let identical = {
            let mut local = self.tracker.lock();
            local
                .iter()
                .position(|(_, text)| message_text.eq(text))
                .and_then(|index| local.swap_remove_back(index))
                .map(|(id, _)| id)
        };
        let similar = || {
            let index = self
                .tracker
                .lock()
                .iter()
                .enumerate()
                .map(|(index, (_, text))| (index, strsim::levenshtein(message_text, text)))
                .min_by_key(|(_, distance)| *distance)
                .map(|(index, _)| index);
            index
                .and_then(|index| self.tracker.lock().swap_remove_back(index))
                .map(|(id, _)| id)
        };

        let to_remove = identical.or_else(similar);

        self.messages.update(|messages| {
            if let Some(id) = to_remove {
                messages.shift_remove(&id).unwrap();
            }
        });
    }
}
