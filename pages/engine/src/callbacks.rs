use leptos::{
    prelude::window,
    web_sys::{
        self,
        wasm_bindgen::prelude::{Closure, JsValue, wasm_bindgen},
    },
};

macro_rules! callback {
    ($name: expr, $func: ident, $type: ty) => {
        ($name, Closure::<$type>::new($func).into_js_value())
    };
}

pub fn setup() {
    let callbacks = &[
        callback!("onRequestFile", on_request_file, dyn Fn(_)),
        callback!("onUpdateSystemGraphic", on_update_system_graphic, dyn Fn(_)),
        callback!(
            "onUpdateConnectionStatus",
            on_update_connection_status,
            dyn Fn(_)
        ),
        callback!("onLoadMap", on_load_map, dyn Fn(_)),
        callback!(
            "onPlayerSpriteUpdated",
            on_player_sprite_updated,
            dyn Fn(_, _, _)
        ),
        callback!("onPlayerTeleported", on_player_teleported, dyn Fn(_, _, _)),
        callback!("syncPlayerData", sync_player_data, dyn Fn(_, _, _, _, _, _)),
        callback!("onRoomSwitch", on_room_switch, dyn Fn()),
        callback!(
            "shouldConnectPlayer",
            should_connect_player,
            dyn Fn(_) -> bool
        ),
        callback!(
            "onPlayerConnectedOrUpdated",
            on_player_connected_or_updated,
            dyn Fn(_, _, _)
        ),
        callback!("onPlayerDisconnected", on_player_disconnected, dyn Fn(_)),
        callback!("onNametagModeUpdated", on_nametag_mode_updated, dyn Fn(_)),
        callback!("onSaveSlotUpdated", on_save_slot_updated, dyn Fn(_)),
    ];

    let window = window();
    for (key, callback) in callbacks {
        use web_sys::js_sys::Reflect;

        Reflect::set(&window, &JsValue::from_str(key), callback).unwrap();
    }
}

#[allow(clippy::needless_pass_by_value)]
#[wasm_bindgen]
pub fn on_request_file(_file: String) {
    // this callback is only used for preloads on forest-orb
}

#[allow(clippy::needless_pass_by_value)]
#[wasm_bindgen]
pub fn on_update_system_graphic(graphic: String) {
    leptos::logging::log!("changing system graphic: {graphic}");
}

#[wasm_bindgen]
pub fn on_update_connection_status(status: u32) {
    use common::messages::play::ConnectionStatus;
    crate::messages::send(common::PlayMessage::ConnectionStatusUpdated(match status {
        0 => ConnectionStatus::Disconnected,
        1 => ConnectionStatus::Connected,
        2 => ConnectionStatus::Connecting,
        _ => panic!("Invalid connection state sent to callback"),
    }));
}

#[allow(clippy::needless_pass_by_value)]
#[wasm_bindgen]
pub fn on_load_map(map: String) {
    leptos::logging::log!("loading map: {map}");
}

#[allow(clippy::needless_pass_by_value)]
#[wasm_bindgen]
pub fn on_player_sprite_updated(sprite: String, index: u32, id: i32) {
    leptos::logging::log!("changed {id}'s sprite to {sprite}/{index}");
}

#[wasm_bindgen]
pub fn on_player_teleported(map: u32, x: u32, y: u32) {
    leptos::logging::log!("teleported to Map{map:<04} X:{x} Y:{y}");
}

#[wasm_bindgen]
pub fn sync_player_data(
    uuid: String,
    rank: u32,
    account: bool,
    badge: String,
    medals: Vec<u32>,
    id: u32,
) {
    crate::messages::send(common::PlayMessage::SyncPlayerData(
        uuid,
        rank,
        account,
        badge,
        medals.try_into().unwrap(),
        id,
    ));
}

#[wasm_bindgen]
pub fn on_room_switch() {
    leptos::logging::log!("room switched");
}

#[wasm_bindgen]
pub fn should_connect_player(_uuid: String) -> bool {
    // blocking, but other people can still see you even if you return false
    true
}

#[wasm_bindgen]
pub fn on_player_connected_or_updated(_system: String, name: String, id: u32) {
    leptos::logging::log!("connected {name} as id {id}");
}

#[wasm_bindgen]
pub fn on_player_disconnected(id: u32) {
    leptos::logging::log!("player with {id} disconnected");
}

#[wasm_bindgen]
pub fn on_nametag_mode_updated(_mode: u32) {
    // why is this even a callback
}

pub fn on_save_slot_updated(slot: u32) {
    leptos::logging::log!("saved to slot {slot}");
}
