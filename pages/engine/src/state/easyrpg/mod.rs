use leptos::{prelude::*, reactive::send_wrapper_ext::SendOption, wasm_bindgen::JsValue};

use crate::state::easyrpg::bindings::{PlayerJSObject, create_easyrpg_player};

mod bindings;
mod configuration;

pub use configuration::Configuration;

pub struct Player {
    pub object: ReadSignal<SendOption<PlayerJSObject>>,
    set_object: WriteSignal<SendOption<PlayerJSObject>>,
}

impl Default for Player {
    fn default() -> Self {
        let (object, set_object) = signal(SendOption::new_local(None::<PlayerJSObject>));

        Effect::new(move || {
            let value = object.with(move |object| JsValue::from(object.as_ref()));
            leptos::web_sys::js_sys::Reflect::set(
                &window(),
                &JsValue::from_str("easyrpgPlayer"),
                &value,
            )
            .unwrap();
        });

        Self { object, set_object }
    }
}

impl Player {
    #[allow(clippy::future_not_send)]
    pub async fn start(&self, config: Configuration) {
        let object = create_easyrpg_player(serde_wasm_bindgen::to_value(&config).unwrap()).await;
        object.init_api();
        self.set_object.set(SendOption::new_local(Some(object)));
    }

    pub fn call<F>(&self, closure: F)
    where
        F: FnOnce(&PlayerJSObject),
    {
        let object = self.object.read();
        if let Some(object) = &**object {
            closure(object);
        }
    }

    pub fn call_untracked<F>(&self, closure: F)
    where
        F: FnOnce(&PlayerJSObject),
    {
        let object = self.object.read_untracked();
        if let Some(object) = &**object {
            closure(object);
        }
    }
}
