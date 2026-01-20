use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

#[wasm_bindgen]
extern "C" {
    // (window.)easyrpgPlayer
    #[wasm_bindgen]
    pub type PlayerJSObject;

    #[wasm_bindgen(js_name = createEasyRpgPlayer)]
    pub async fn create_easyrpg_player(config: JsValue) -> PlayerJSObject;

    #[wasm_bindgen(method, js_name = initApi)]
    pub fn init_api(this: &PlayerJSObject);

    #[wasm_bindgen(method, getter)]
    pub fn api(this: &PlayerJSObject) -> PlayerAPIJSObject;

    // (window.)easyrpgPlayer.api
    #[wasm_bindgen]
    pub type PlayerAPIJSObject;

    #[wasm_bindgen(method, js_name = sessionReady)]
    pub fn session_ready(this: &PlayerAPIJSObject);
}
