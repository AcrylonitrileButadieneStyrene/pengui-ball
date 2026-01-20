pub mod engine;
pub mod play;

use web_sys::wasm_bindgen::JsCast as _;

fn ser<T: serde::Serialize>(value: T) -> web_sys::wasm_bindgen::JsValue {
    let bytes = postcard::to_allocvec(&value).unwrap();
    web_sys::js_sys::Uint8Array::new_from_slice(&bytes)
        .buffer()
        .into()
}

fn de<T: for<'a> serde::Deserialize<'a>>(buffer: web_sys::wasm_bindgen::JsValue) -> Option<T> {
    let data = buffer.dyn_into::<web_sys::js_sys::ArrayBuffer>().ok()?;
    let data = web_sys::js_sys::Uint8Array::new(&data).to_vec();
    postcard::from_bytes(&data).ok()
}

macro_rules! impl_methods {
    ($t: ty) => {
        impl $t {
            pub fn ser(&self) -> web_sys::wasm_bindgen::JsValue {
                ser(self)
            }

            pub fn de(buffer: web_sys::wasm_bindgen::JsValue) -> Option<Self> {
                de(buffer)
            }
        }
    };
}

impl_methods!(engine::Message);
impl_methods!(play::Message);
