use std::sync::Arc;

use itertools::Itertools;
use leptos::prelude::*;

use crate::{
    sidebar::chat::message::components::{
        global::GlobalMessage, map::MapMessage, party::PartyMessage,
    },
    state::chat::message::MessageItem,
    states::{
        locations::Location,
        players::{friend::Friend, player::PlayerStoreFields as _},
    },
};

pub fn on_message(state: &crate::state::PlayState, parts: &[&str]) {
    match parts {
        ["pc", count] => state.players.count.set(count.parse::<u32>().ok()),
        ["say", uuid, text] => state.chat.add(
            MessageItem::new(
                None::<&str>,
                Arc::from(*text),
                state.chat.channel::<MapMessage>().filter.read_only(),
            ),
            MapMessage {
                author: Arc::from(*uuid),
            },
        ),
        ["psay", uuid, text, id] => state.chat.add(
            MessageItem::new(
                Some(*id),
                Arc::from(*text),
                state.chat.channel::<PartyMessage>().filter.read_only(),
            ),
            PartyMessage {
                author: Arc::from(*uuid),
            },
        ),
        ["gsay", uuid, map, prev, _, x, y, text, id] => {
            state.chat.add(
                MessageItem::new(
                    Some(*id),
                    Arc::from(*text),
                    state.chat.channel::<GlobalMessage>().filter.read_only(),
                ),
                GlobalMessage {
                    author: Arc::from(*uuid),
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
                            leptos::logging::warn!(
                                "Chat message parse error: {map},{prev},{x},{y}"
                            );
                            None
                        }
                    },
                },
            );
        }
        ["p", uuid, name, system, rank, account, badge, medals @ ..] => {
            let uuid = Arc::from(*uuid);

            let player = state.players.get_or_init(&uuid, false);
            player.name().set(Some(Arc::from(*name)));
            player.system().set(Some(Arc::from(*system)));
            player.rank().set(rank.parse().unwrap());
            player.account().set((*account).eq("1"));
            player.badge().set(match *badge {
                "null" => None,
                _ => Some(Arc::from(*badge)),
            });
            player.medals().set(
                medals
                    .iter()
                    .map(|medal| medal.parse().unwrap_or_default())
                    .collect::<Vec<_>>()
                    .try_into()
                    .inspect_err(|err| {
                        leptos::logging::error!("Error while parsing player medals: {err:?}");
                    })
                    .unwrap_or([0, 0, 0, 0, 0]),
            );
        }
        ["e", json] => {
            state.expeds.set(serde_json::from_str(json).ok());
        }
        ["eec", experience, is_ok] => {
            if *is_ok != "0" {
                leptos::logging::log!("completed exped for {experience} xp");
                state
                    .session
                    .channel
                    .send(crate::sidebar::session::Command::GetExpeds)
                    .unwrap();
            } else {
                leptos::logging::warn!("received error when claiming exped");
            }
        }
        ["vm", experience] => {
            leptos::logging::log!("completed vm for {experience} xp");
            state
                .session
                .channel
                .send(crate::sidebar::session::Command::GetExpeds)
                .unwrap();
        }
        ["pf", json] => {
            let new = serde_json::from_str::<Vec<Friend>>(json)
                .unwrap()
                .into_iter()
                .sorted_by_key(|friend| friend.name.clone())
                .sorted_by_key(|friend| friend.online)
                .collect();
            state.players.friends.update(|players| *players = new);
        }
        [cmd, args @ ..] => {
            leptos::logging::warn!("Received unknown command \"{cmd}\" with args {args:?}");
        }
        [] => leptos::logging::warn!("Received empty command"),
    }
}
