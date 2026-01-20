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
    leptos::logging::log!("changed {sprite}'s sprite to '{index}' #{id}");
}

#[wasm_bindgen]
pub fn on_player_teleported(map: u32, x: u32, y: u32) {
    leptos::logging::log!("teleported to Map{map:<04} X:{x} Y:{y}");
}
