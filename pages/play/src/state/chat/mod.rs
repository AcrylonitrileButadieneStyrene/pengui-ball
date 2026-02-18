use std::sync::Arc;

use leptos::prelude::*;

mod channel;
mod destination;
mod message;

pub use channel::ChatChannel;
pub use destination::MessageDestination;
pub use message::{Message, MessageData};

pub struct State {
    pub messages: ReadSignal<indexmap::IndexMap<Arc<str>, Message>>,

    pub map: ChatChannel,
    pub party: ChatChannel,
    pub global: ChatChannel,
    pub sending: ChatChannel,

    pub input: NodeRef<leptos::html::Div>,
    pub destination: RwSignal<MessageDestination>,

    /// User ID of the currently signed in user.
    my_id: Signal<Option<Arc<str>>>,
}

impl State {
    pub fn new(my_id: Signal<Option<Arc<str>>>) -> Self {
        let (messages, set_messages) = signal(indexmap::IndexMap::new());
        Self {
            messages,
            map: ChatChannel::new(set_messages, 150, false),
            party: ChatChannel::new(set_messages, 150, false),
            global: ChatChannel::new(set_messages, 150, false),
            sending: ChatChannel::new(set_messages, 10, false),
            input: NodeRef::new(),
            destination: RwSignal::default(),
            my_id,
        }
    }

    pub fn add(&self, message: Message) {
        match &message.data {
            MessageData::Map { author, .. } => {
                if self.is_self(author) {
                    self.sending.remove(&message);
                }
                self.map.add(message);
            }
            MessageData::Party { author, .. } => {
                if self.is_self(author) {
                    self.sending.remove(&message);
                }
                self.party.add(message);
            }
            MessageData::Global { author, .. } => {
                if self.is_self(author) {
                    self.sending.remove(&message);
                }
                self.global.add(message);
            }
            MessageData::Sending { .. } => {
                self.sending.add(message);
            }
        }
    }

    fn is_self(&self, author: &str) -> bool {
        self.my_id
            .read_untracked()
            .as_ref()
            .is_some_and(|id| &**id == author)
    }
}
