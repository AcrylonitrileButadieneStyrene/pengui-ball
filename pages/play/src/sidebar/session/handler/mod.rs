use std::sync::Arc;

use itertools::Itertools;
use leptos::prelude::*;

use crate::{
    sidebar::chat::message::components::{
        global::GlobalMessage, map::MapMessage, party::PartyMessage,
    },
    states::players::{friend::Friend, player::PlayerStoreFields},
};

mod chat;

pub fn on_message(state: &crate::state::PlayState, parts: &[&str]) {
    match parts {
        ["pc", count] => state.players.count.set(count.parse::<u32>().ok()),
        ["say", uuid, text] => chat::say(state, chat::item::<MapMessage>(state, text, None), uuid),
        ["psay", uuid, text, id] => chat::psay(
            state,
            chat::item::<PartyMessage>(state, text, Some(id)),
            uuid,
        ),
        ["gsay", uuid, map, prev, _, x, y, text, id] => {
            chat::gsay(
                state,
                chat::item::<GlobalMessage>(state, text, Some(id)),
                uuid,
                map,
                prev,
                x,
                y,
            );
        }
        ["p", uuid, name, system, rank, account, badge, medals @ ..] => {
            let player = state.players.get_or_init(&Arc::from(*uuid), false);
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
            if *is_ok == "0" {
                leptos::logging::warn!("received error when claiming exped");
            } else {
                leptos::logging::log!("completed exped for {experience} xp");
                state
                    .session
                    .channel
                    .send(crate::sidebar::session::Command::GetExpeds)
                    .unwrap();
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
