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
        common::PlayMessage::SyncPlayerData(uuid, rank, account, badge, medals, id) => {
            let uuid = Arc::<str>::from(uuid);
            state
                .uuids
                .update(|uuids| drop(uuids.insert(id, uuid.clone())));
            state.players.update(|players| {
                let badge = match &*badge {
                    "null" => None,
                    _ => Some(Arc::from(badge)),
                };

                if let Some(player) = players.get_mut(&uuid) {
                    player.rank = rank;
                    player.account = account;
                    player.badge = badge;
                    player.medals = medals;
                } else {
                    players.insert(
                        uuid,
                        crate::state::Player {
                            name: None,
                            system: None,
                            rank,
                            account,
                            badge,
                            medals,
                        },
                    );
                }
            });
        }
    }
}
