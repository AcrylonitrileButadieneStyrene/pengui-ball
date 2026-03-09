use std::sync::Arc;

use crate::{
    sidebar::chat::message::components::{
        global::GlobalMessage, map::MapMessage, party::PartyMessage,
    },
    state::chat::message::{MessageComponent, MessageItem},
    states::locations::Location,
};

pub fn item<T: MessageComponent + 'static>(
    state: &crate::state::PlayState,
    text: &str,
    id: Option<&str>,
) -> MessageItem {
    MessageItem::new(
        id,
        Arc::from(text),
        state.chat.channel::<T>().filter.read_only(),
    )
}

pub fn say(state: &crate::state::PlayState, item: MessageItem, uuid: &str) {
    state.chat.add(
        item,
        MapMessage {
            author: Arc::from(uuid),
        },
    );
}

pub fn psay(state: &crate::state::PlayState, item: MessageItem, uuid: &str) {
    state.chat.add(
        item,
        PartyMessage {
            author: Arc::from(uuid),
        },
    );
}

pub fn gsay(
    state: &crate::state::PlayState,
    item: MessageItem,
    uuid: &str,
    map: &str,
    prev: &str,
    x: &str,
    y: &str,
) {
    state.chat.add(
        item,
        GlobalMessage {
            author: Arc::from(uuid),
            location: {
                if let Ok(map) = map.parse()
                    && let Ok(x) = x.parse()
                    && let Ok(y) = y.parse()
                {
                    Some(Location {
                        game: state.locations.game.clone(),
                        map,
                        previous: prev.parse().ok(),
                        x,
                        y,
                    })
                } else {
                    leptos::logging::warn!("Chat message parse error: {map},{prev},{x},{y}");
                    None
                }
            },
        },
    );
}
