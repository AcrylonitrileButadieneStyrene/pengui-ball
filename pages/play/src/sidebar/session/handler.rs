use leptos::prelude::{Set, Update};

use crate::state::{Message, Player};

pub fn on_message(state: &crate::state::State, parts: &[&str]) {
    match parts {
        ["pc", count] => state.player_count.set(count.parse::<u32>().ok()),
        ["say", uuid, txt] => state.chat.add_map(Message::new("", uuid, txt)),
        ["psay", uuid, txt, id] => state.chat.add_party(Message::new(id, uuid, txt)),
        ["gsay", uuid, _map, _, _, _x, _y, txt, id] => {
            state.chat.add_global(Message::new(id, uuid, txt))
        }
        ["p", uuid, name, system, rank, account, badge, medals @ ..] => {
            let uuid = uuid.to_string();
            let new_player = Player {
                name: name.to_string(),
                system: system.to_string(),
                rank: rank.parse().unwrap(),
                account: (*account).eq("1"),
                badge: match *badge {
                    "null" => None,
                    _ => Some(badge.to_string()),
                },
                medals: medals
                    .into_iter()
                    .map(|medal| medal.parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            };

            state.players.update(|players| {
                players.insert(uuid, std::sync::Arc::new(new_player));
            });
        }
        [cmd, args @ ..] => {
            leptos::logging::warn!("Received unknown command \"{cmd}\" with args {args:?}")
        }
        [] => leptos::logging::warn!("Received empty command"),
    }
}
