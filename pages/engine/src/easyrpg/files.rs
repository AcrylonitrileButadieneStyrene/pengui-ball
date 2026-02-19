use std::sync::Arc;

use idb::DatabaseEvent as _;
use leptos::{
    wasm_bindgen::JsValue,
    web_sys::js_sys::{Reflect, Uint8Array},
};

pub fn get_file(state: &crate::EngineState, id: usize) {
    let game = state.game.clone();
    leptos::task::spawn_local(async move {
        let factory = idb::Factory::new().unwrap();
        let database = factory
            .open(&format!("/easyrpg/{game}/Save"), None)
            .unwrap()
            .await
            .unwrap();
        let transaction = database
            .transaction(&["FILE_DATA"], idb::TransactionMode::ReadOnly)
            .unwrap();
        let store = transaction.object_store("FILE_DATA").unwrap();
        let value = store
            .get(idb::Query::Key(
                format!("/easyrpg/{game}/Save/Save{id:>02}.lsd").into(),
            ))
            .unwrap()
            .await
            .unwrap()
            .unwrap();
        let file: File = serde_wasm_bindgen::from_value(value).unwrap();
        crate::send(common::PlayMessage::SaveData(id, file.into()));
    });
}

pub fn set_file(state: &crate::EngineState, id: usize, data: Vec<u8>) {
    leptos::logging::log!("uploading save with {} bytes", data.len());
}

pub fn delete_file(state: &crate::EngineState, id: usize) {}

#[derive(serde::Deserialize)]
struct File {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub timestamp: leptos::web_sys::js_sys::Date,
    pub contents: Arc<[u8]>,
}

impl From<File> for common::messages::play::SaveFile {
    fn from(value: File) -> Self {
        Self {
            contents: value.contents,
            timestamp: value.timestamp.to_iso_string().into(),
        }
    }
}
