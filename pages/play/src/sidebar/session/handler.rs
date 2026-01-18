use leptos::prelude::Set;

use crate::sidebar::chat::Message;

pub fn on_message(state: &crate::state::State, parts: &[&str]) {
    match parts {
        ["pc", count] => state.player_count.set(count.parse::<u32>().ok()),
        ["say", uuid, txt] => state.chat.add_map(Message::new("", uuid, txt)),
        ["psay", uuid, txt, id] => state.chat.add_party(Message::new(id, uuid, txt)),
        ["gsay", uuid, _map, _, _, _x, _y, txt, id] => {
            state.chat.add_global(Message::new(id, uuid, txt))
        }
        [cmd, args @ ..] => leptos::logging::warn!("Received command {cmd} with args: {args:?}"),
        [] => leptos::logging::warn!("Received empty command"),
    }
}
