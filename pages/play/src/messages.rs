use std::sync::Arc;

use leptos::{ev, prelude::*};

#[island]
pub fn Handler() -> impl IntoView {
    let state = expect_context::<Arc<crate::State>>();

    window_event_listener(ev::message, move |ev| {
        let Some(message) = common::PlayMessage::de(ev.data()) else {
            return;
        };

        handle(&state, message);
    });
}

fn handle(state: &crate::State, message: common::PlayMessage) {
    match message {
        common::PlayMessage::ConnectionStatusUpdated(status) => {
            state.engine.set_status(status);
        }
        common::PlayMessage::PlayerSync(common::messages::play::PlayerSyncData {
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
            });
        }
        common::PlayMessage::PlayerConnect(common::messages::play::PlayerConnectData {
            id,
            name,
            system,
        }) => {
            state.players.with_untracked(|players| {
                let uuids = state.players.uuids.read_untracked();
                let Some(uuid) = uuids.get(&id) else {
                    return;
                };

                let Some(player) = players.get(uuid) else {
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
    }
}
