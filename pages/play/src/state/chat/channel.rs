use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use leptos::prelude::*;

use crate::state::MessageData;

use super::Message;

pub struct ChatChannel {
    pub capacity: RwSignal<usize>,
    /// List of IDs corresponding to messages in the primary `messages` list
    pub tracker: Mutex<VecDeque<Arc<str>>>,
    /// If true then this channel is not shown to the user
    pub filter: RwSignal<bool>,
    /// List of local messages sent to this channel not yet echoed by the server
    pub local: Mutex<Vec<(Arc<str>, Arc<str>)>>,
    /// Shared message list, all channels interact with it.
    messages: WriteSignal<indexmap::IndexMap<Arc<str>, Message>>,
    /// User ID of the currently signed in user.
    my_id: Signal<Option<Arc<str>>>,
}

impl ChatChannel {
    pub fn new(
        messages: WriteSignal<indexmap::IndexMap<Arc<str>, Message>>,
        my_id: Signal<Option<Arc<str>>>,
        capacity: usize,
        filtered: bool,
    ) -> Self {
        Self {
            capacity: RwSignal::new(capacity),
            tracker: Mutex::new(VecDeque::with_capacity(capacity)),
            filter: RwSignal::new(filtered),
            local: Mutex::new(Vec::new()),
            messages,
            my_id,
        }
    }

    pub fn add(&self, mut message: Message) {
        leptos::logging::log!("adding {message:?}");

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

            let removed_local = self.remove_local(&message);

            self.messages.update(|messages| {
                if let Some(id) = removed_overflow {
                    messages.shift_remove(&id).unwrap();
                }

                leptos::logging::log!("removing {removed_local:?}");
                if let Some(id) = removed_local {
                    messages.shift_remove(&id).unwrap();
                }

                messages.insert(message.id.clone(), message);
            });
        }
    }

    fn remove_local(&self, message: &Message) -> Option<Arc<str>> {
        let author = match &message.data {
            MessageData::Map { author, .. }
            | MessageData::Party { author, .. }
            | MessageData::Global { author, .. } => author,
            _ => return None,
        };

        if !self
            .my_id
            .read_untracked()
            .as_ref()
            .is_some_and(|id| id == author)
        {
            return None;
        }

        let message_text = message.text();
        let mut local = self.local.lock().unwrap();
        let identical = local
            .iter()
            .position(|(_, text)| message_text.eq(text))
            .map(|index| local.swap_remove(index).0);
        let similar = || {
            let index = local
                .iter()
                .enumerate()
                .map(|(index, (_, text))| (index, strsim::levenshtein(message_text, text)))
                .min_by_key(|(_, distance)| *distance)
                .map(|(index, _)| index);
            index.map(|index| local.swap_remove(index).0)
        };

        identical.or_else(similar)
    }
}
