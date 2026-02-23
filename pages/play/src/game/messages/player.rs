use std::sync::Arc;

use common::messages::play::{PlayerConnectData, PlayerSyncData};
use leptos::prelude::*;

use crate::state::game::Location;

pub fn sync(state: &crate::state::PlayState, data: PlayerSyncData) {
    let PlayerSyncData {
        uuid,
        rank,
        account,
        badge,
        medals,
        id,
    } = data;

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
            && let Some(Ok(user)) = &*state.api.user.read_untracked()
        {
            player.name = Some(user.name.clone().into());
        }
    });
}

pub fn connect(state: &crate::state::PlayState, data: PlayerConnectData) {
    let PlayerConnectData { id, name, system } = data;

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

pub fn teleported(state: &crate::state::PlayState, map: u16, x: i16, y: i16) {
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
