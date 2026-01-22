use std::sync::Arc;

use leptos::{ev, prelude::*};

#[island]
pub fn Handler() -> impl IntoView {
    let state = use_context::<Arc<crate::State>>().unwrap();

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
                .uuids
                .update(|uuids| drop(uuids.insert(id, uuid.clone())));

            let badge = match &*badge {
                "null" => None,
                _ => Some(Arc::from(badge)),
            };

            // get player, create new if neeeded, then update
            state
                .players
                .with(|players| players.get(&uuid).cloned())
                .unwrap_or_else(|| {
                    let signal = RwSignal::new(crate::state::Player::default());
                    state
                        .players
                        .update(|players| assert!(players.insert(uuid.clone(), signal).is_none()));
                    signal
                })
                .update(|player| {
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
            state.players.with(|players| {
                let uuids = state.uuids.read_untracked();
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
