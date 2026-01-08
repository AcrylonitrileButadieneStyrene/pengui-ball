use leptos::prelude::window;
use web_sys::wasm_bindgen::prelude::{Closure, JsValue, wasm_bindgen};

macro_rules! callback {
    ($name: expr, $func: ident, $type: ty) => {
        ($name, Closure::<$type>::new($func).into_js_value())
    };
}

pub fn setup() {
    let callbacks = &[
        callback!("onRequestFile", on_request_file, dyn Fn(_)),
        callback!("onUpdateSystemGraphic", on_update_system_graphic, dyn Fn(_)),
    ];

    let window = window();
    for (key, callback) in callbacks {
        use web_sys::js_sys::Reflect;

        Reflect::set(&window, &JsValue::from_str(key), callback).unwrap();
    }
}

#[wasm_bindgen]
pub fn on_request_file(file: String) {
    leptos::logging::log!("loading file: {file:?}");
}

#[wasm_bindgen]
pub fn on_update_system_graphic(graphic: String) {
    leptos::logging::log!("changing system graphic: {graphic:?}");
}
