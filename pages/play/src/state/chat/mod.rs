use std::{any::TypeId, collections::HashMap, sync::Arc};

use leptos::prelude::*;

mod channel;
mod destination;
pub mod message;

pub use channel::ChatChannel;
pub use destination::MessageDestination;

use crate::state::chat::message::{MessageItem, MessageType};

type MessageList = indexmap::IndexMap<Arc<str>, (MessageItem, Arc<dyn MessageType>)>;

pub struct State {
    pub messages: ReadSignal<MessageList>,
    set_messages: WriteSignal<MessageList>,
    channels: RwSignal<HashMap<TypeId, Arc<ChatChannel>>>,

    pub input: NodeRef<leptos::html::Div>,
    pub destination: RwSignal<MessageDestination>,

    pub guest_name: RwSignal<Option<Arc<str>>>,
    /// User ID of the currently signed in user.
    pub my_id: Signal<Option<Arc<str>>>,
}

impl State {
    pub fn new(my_id: Signal<Option<Arc<str>>>) -> Self {
        let (messages, set_messages) = signal(indexmap::IndexMap::new());
        Self {
            messages,
            set_messages,
            channels: RwSignal::default(),
            input: NodeRef::new(),
            destination: RwSignal::default(),
            guest_name: RwSignal::default(),
            my_id,
        }
    }

    pub fn channel<T: MessageType + 'static>(&self) -> Arc<ChatChannel> {
        let id = TypeId::of::<T>();
        let channel = {
            let channels = self.channels.read_untracked();
            channels.get(&id).cloned()
        };
        channel.unwrap_or_else(|| {
            let channel = Arc::new(ChatChannel::new(self.set_messages));
            self.channels.update(|channels| {
                channels.insert(id, channel.clone());
            });
            channel
        })
    }

    pub fn add<T: MessageType + 'static>(&self, message: MessageItem, data: T) {
        data.on_add(&message, self);

        self.channel::<T>().add(message, Arc::new(data));
    }
}
