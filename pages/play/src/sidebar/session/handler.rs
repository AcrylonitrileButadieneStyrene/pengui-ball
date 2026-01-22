use std::sync::Arc;

use leptos::prelude::{Set, Update};

use crate::state::{Message, MessageData, Player};

pub fn on_message(state: &crate::state::State, parts: &[&str]) {
    match parts {
        ["pc", count] => state.player_count.set(count.parse::<u32>().ok()),
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
            let new_player = Player {
                name: Some(Arc::from(*name)),
                system: Some(Arc::from(*system)),
                rank: rank.parse().unwrap(),
                account: (*account).eq("1"),
                badge: match *badge {
                    "null" => None,
                    _ => Some(Arc::from(*badge)),
                },
                medals: medals
                    .iter()
                    .map(|medal| medal.parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            };

            state.players.update(|players| {
                players.insert(uuid, new_player);
            });
        }
        [cmd, args @ ..] => {
            leptos::logging::warn!("Received unknown command \"{cmd}\" with args {args:?}");
        }
        [] => leptos::logging::warn!("Received empty command"),
    }
}
