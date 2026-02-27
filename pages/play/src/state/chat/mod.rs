use std::{any::TypeId, collections::HashMap, sync::Arc};

use leptos::{prelude::*, reactive::send_wrapper_ext::SendOption};

mod channel;
mod destination;
pub mod message;

pub use channel::ChatChannel;
pub use destination::MessageDestination;

use crate::state::chat::message::{MessageComponent, MessageItem};

type MessageList = indexmap::IndexMap<Arc<str>, (MessageItem, Arc<dyn MessageComponent>)>;

pub struct State {
    pub messages: ReadSignal<MessageList>,
    set_messages: WriteSignal<MessageList>,
    channels: RwSignal<HashMap<TypeId, Arc<ChatChannel>>>,

    pub input: NodeRef<leptos::html::Div>,
    pub destination: RwSignal<MessageDestination>,

    pub guest_name: RwSignal<Option<Arc<str>>>,
    /// User ID of the currently signed in user.
    pub my_id: Signal<Option<Arc<str>>>,
    pub mention_audio: SendOption<leptos::web_sys::HtmlAudioElement>,
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
            mention_audio: SendOption::new_local(is_browser().then(|| {
                let audio =
                    leptos::web_sys::HtmlAudioElement::new_with_src("/audio/mention.wav").unwrap();
                audio.set_preload("auto");
                audio
            })),
        }
    }

    pub fn channel<T: MessageComponent + 'static>(&self) -> Arc<ChatChannel> {
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

    pub fn add<T: MessageComponent + 'static>(&self, message: MessageItem, data: T) {
        self.channel::<T>().add(message, Arc::new(data));
    }
}
