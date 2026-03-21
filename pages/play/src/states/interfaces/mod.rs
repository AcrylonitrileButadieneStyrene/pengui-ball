use leptos::{prelude::*, wasm_bindgen::JsValue, web_sys::js_sys::Reflect};

mod emojis;

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone)]
pub struct Interfaces {
    #[wasm_bindgen(getter_with_clone)]
    pub emojis: emojis::Emojis,
}

impl Interfaces {
    pub fn new() -> Self {
        let instance = Self {
            emojis: emojis::Emojis::new(),
        };

        if is_browser() {
            Reflect::set(
                &window(),
                &"interface".into(),
                &JsValue::from(instance.clone()),
            )
            .unwrap();
        }

        instance
    }
}
