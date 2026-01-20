use leptos::{prelude::*, reactive::send_wrapper_ext::SendOption};

pub struct EasyRPGPlayer {
    pub object: ReadSignal<SendOption<PlayerJSObject>>,
    set_object: WriteSignal<SendOption<PlayerJSObject>>,
}

impl std::ops::Deref for EasyRPGPlayer {
    type Target = ReadSignal<SendOption<PlayerJSObject>>;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl Default for EasyRPGPlayer {
    fn default() -> Self {
        let (object, set_object) = signal(SendOption::new_local(None::<PlayerJSObject>));

        Effect::new(move || {
            let value = object.with(move |object| wasm_bindgen::JsValue::from(object.as_ref()));
            leptos::web_sys::js_sys::Reflect::set(
                &window(),
                &wasm_bindgen::JsValue::from_str("easyrpgPlayer"),
                &value,
            )
            .unwrap();
        });

        Self { object, set_object }
    }
}

impl EasyRPGPlayer {
    pub async fn start(&self) {
        let object = create_easyrpg_player().await;
        object.init_api();
        self.set_object.set(SendOption::new_local(Some(object)));
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type PlayerJSObject;

    #[wasm_bindgen(js_name = createEasyRpgPlayer)]
    pub async fn create_easyrpg_player() -> PlayerJSObject;

    #[wasm_bindgen(method, js_name = initApi)]
    pub fn init_api(this: &PlayerJSObject);
}
