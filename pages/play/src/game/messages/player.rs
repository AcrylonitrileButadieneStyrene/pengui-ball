use std::sync::Arc;

use common::messages::play::{PlayerConnectData, PlayerSyncData};
use leptos::prelude::*;

use crate::states::{locations::Location, players::player::PlayerStoreFields};

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
    let badge = match &*badge {
        "null" => None,
        _ => Some(Arc::from(badge)),
    };

    let player = state.players.get_or_init(&uuid, id == -1);
    player.uuid().set(Some(uuid));
    player.rank().set(rank);
    player.account().set(account);
    player.badge().set(badge);
    player.medals().set(medals);

    if id == -1
        && let Some(Ok(user)) = &*state.api.user.read_untracked()
        && !user.name.is_empty()
    {
        player.name().set(Some(user.name.clone()));
    }

    state.players.in_map.update(|players| {
        players.insert(id.try_into().unwrap(), player);
    });
}

pub fn connect(state: &crate::state::PlayState, data: PlayerConnectData) {
    let PlayerConnectData { id, name, system } = data;

    let Some(player) = state.players.get_by_id(id) else {
        leptos::logging::warn!("connected player does not already exist");
        return;
    };

    if !name.is_empty() {
        player.name().set(Some(name.into()));
    }

    if !system.is_empty() {
        player.system().set(Some(system.into()));
    }
}

pub fn disconnect(state: &crate::state::PlayState, id: i32) {
    state.players.in_map.update(|uuids| {
        uuids.remove(&id.try_into().unwrap());
    });
}

pub fn teleported(state: &crate::state::PlayState, map: u16, x: i16, y: i16) {
    state.locations.current.update(|value| {
        let previous = value.as_ref().map(|previous| previous.map);
        *value = Some(Location {
            game: state.locations.game.clone(),
            map,
            previous,
            x,
            y,
        });
    });
}

pub fn sprite_update(state: &crate::state::PlayState, id: i32, charset: String, index: u8) {
    let sprite = match &*charset {
        "" => None,
        _ => Some((charset.into(), index)),
    };

    if let Some(player) = state.players.get_by_id(id) {
        player.sprite().set(sprite);
    } else {
        leptos::logging::warn!("no player for {id}");
    }
}
