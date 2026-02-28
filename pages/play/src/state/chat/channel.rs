use std::{
    collections::VecDeque,
    sync::{Arc, nonpoison::Mutex},
};

use leptos::prelude::*;

use crate::state::chat::message::MessageComponent;

use super::MessageItem;

pub struct ChatChannel {
    pub capacity: RwSignal<usize>,
    /// List of IDs corresponding to messages in the primary `messages` list
    pub tracker: Mutex<VecDeque<(Arc<str>, Arc<str>)>>,
    /// If true then this channel is not shown to the user
    pub filter: RwSignal<bool>,
    /// Shared message list, all channels interact with it.
    messages: WriteSignal<super::MessageList>,
}

impl ChatChannel {
    pub fn new(messages: WriteSignal<super::MessageList>) -> Self {
        Self {
            capacity: RwSignal::new(150),
            tracker: Mutex::new(VecDeque::with_capacity(150)),
            filter: RwSignal::new(false),
            messages,
        }
    }

    pub fn add(&self, message: MessageItem, data: Arc<dyn MessageComponent>) {
        let mut buffer = self.tracker.lock();
        let removed_overflow = if buffer.len() + 1 > self.capacity.get_untracked() {
            buffer.pop_back()
        } else {
            None
        };

        buffer.push_front((message.id.clone(), message.text.clone()));
        drop(buffer);

        self.messages.update(|messages| {
            if let Some((id, _)) = removed_overflow {
                messages.shift_remove(&id).unwrap();
            }

            messages.insert(message.id.clone(), (message, data));
        });
    }

    pub fn remove(&self, message: &MessageItem) {
        self.remove_by_text(&message.text);
    }

    pub fn remove_by_text(&self, message: &str) {
        let identical = {
            let mut local = self.tracker.lock();
            local
                .iter()
                .position(|(_, text)| message == text.as_ref())
                .and_then(|index| local.swap_remove_back(index))
                .map(|(id, _)| id)
        };
        let similar = || {
            let index = self
                .tracker
                .lock()
                .iter()
                .enumerate()
                .map(|(index, (_, text))| (index, strsim::levenshtein(&message, text)))
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
