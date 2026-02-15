use std::sync::Arc;

use common::{
    PlayMessage,
    messages::play::{PlayerConnectData, PlayerSyncData},
};
use leptos::{ev, prelude::*};
use leptos_use::core::ConnectionReadyState;

use crate::state::game::Location;

pub fn setup_handler(state: Arc<crate::state::PlayState>) {
    window_event_listener(ev::message, move |ev| {
        let Some(message) = common::PlayMessage::de(ev.data()) else {
            return;
        };

        handle(&state, message);
    });
}

fn handle(state: &crate::state::PlayState, message: common::PlayMessage) {
    match message {
        PlayMessage::EngineLoaded => {
            if state.engine.load_count.get_untracked() > 1
                && state.session.status.get_untracked() == ConnectionReadyState::Open
            {
                state.engine.send(common::EngineMessage::Connect);
            }
        }
        PlayMessage::ConnectionStatusUpdated(status) => {
            state.engine.set_status(status);
        }
        PlayMessage::PlayerSync(PlayerSyncData {
            uuid,
            rank,
            account,
            badge,
            medals,
            id,
        }) => {
            let uuid = Arc::<str>::from(uuid);
            state
                .players
                .uuids
                .update(|uuids| drop(uuids.insert(id, uuid.clone())));

            let badge = match &*badge {
                "null" => None,
                _ => Some(Arc::from(badge)),
            };

            state.players.get_or_init(&uuid).update(|player| {
                player.rank = rank;
                player.account = account;
                player.badge = badge;
                player.medals = medals;

                if id == -1
                    && let Some(Some(user)) = &*state.api.user.read_untracked()
                {
                    player.name = Some(user.name.clone().into());
                }
            });
        }
        PlayMessage::PlayerConnect(PlayerConnectData { id, name, system }) => {
            state.players.with_untracked(|players| {
                let uuids = state.players.uuids.read_untracked();
                let Some(uuid) = uuids.get(&id) else {
                    return;
                };
                let uuid = uuid.clone();
                drop(uuids);

                let Some(player) = players.get(&uuid) else {
                    return;
                };

                player.update(|player| {
                    if !name.is_empty() {
                        player.name = Some(name.into());
                    }

                    if !system.is_empty() {
                        player.system = Some(system.into());
                    }
                });
            });
        }
        PlayMessage::RegainFocus(_shift_held) => {
            if let Some(element) = state.chat.input.get_untracked() {
                element.focus().unwrap();
            }
        }
        PlayMessage::PlayerTeleported(map, x, y) => {
            state.game.location.update(|value| {
                let previous = value.as_ref().map(|previous| previous.map);
                *value = Some(Location {
                    game: state.game.id.clone(),
                    map,
                    previous,
                    x,
                    y,
                });
            });
        }
    }
}
