use std::sync::Arc;

use leptos::prelude::*;

use crate::state::{Message, MessageData};

pub fn on_message(state: &crate::state::PlayState, parts: &[&str]) {
    match parts {
        ["pc", count] => state.players.count.set(count.parse::<u32>().ok()),
        ["say", uuid, text] => state.chat.map.add(Message::new(
            None::<&str>,
            MessageData::Map {
                author: Arc::from(*uuid),
                text: Arc::from(*text),
            },
        )),
        ["psay", uuid, text, id] => state.chat.party.add(Message::new(
            Some(*id),
            MessageData::Party {
                author: Arc::from(*uuid),
                text: Arc::from(*text),
            },
        )),
        ["gsay", uuid, _map, _, _, _x, _y, text, id] => {
            state.chat.global.add(Message::new(
                Some(*id),
                MessageData::Global {
                    author: Arc::from(*uuid),
                    text: Arc::from(*text),
                },
            ));
        }
        ["p", uuid, name, system, rank, account, badge, medals @ ..] => {
            let uuid = Arc::from(*uuid);

            state.players.get_or_init(&uuid).update(|player| {
                player.name = Some(Arc::from(*name));
                player.system = Some(Arc::from(*system));
                player.rank = rank.parse().unwrap();
                player.account = (*account).eq("1");
                player.badge = match *badge {
                    "null" => None,
                    _ => Some(Arc::from(*badge)),
                };
                player.medals = medals
                    .iter()
                    .map(|medal| medal.parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
            });
        }
        [cmd, args @ ..] => {
            leptos::logging::warn!("Received unknown command \"{cmd}\" with args {args:?}");
        }
        [] => leptos::logging::warn!("Received empty command"),
    }
}
