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

    pub input: NodeRef<leptos::html::Div>,
    pub destination: RwSignal<MessageDestination>,
}

impl Default for State {
    fn default() -> Self {
        let (messages, set_messages) = signal(indexmap::IndexMap::new());

        Self {
            messages,
            map: ChatChannel::new(set_messages, 150, false),
            party: ChatChannel::new(set_messages, 150, false),
            global: ChatChannel::new(set_messages, 150, false),
            input: NodeRef::new(),
            destination: RwSignal::default(),
        }
    }
}
