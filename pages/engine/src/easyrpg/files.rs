use std::sync::Arc;

use leptos::{
    prelude::{location, window},
    web_sys::{
        js_sys::{Date, Uint8Array},
        wasm_bindgen::JsValue,
    },
};

pub fn get_file(state: &crate::EngineState, id: usize) {
    let game = state.game.clone();
    leptos::task::spawn_local(async move {
        let (transaction, store, key, exists) = get_store(&game, id).await;
        if !exists {
            transaction.abort().unwrap().await.unwrap();
            return;
        }

        let value = store
            .get(idb::Query::Key(key.into()))
            .unwrap()
            .await
            .unwrap()
            .unwrap();
        let file: File = serde_wasm_bindgen::from_value(value).unwrap();
        crate::send(common::PlayMessage::SaveData(id, file.into()));
        transaction.commit().unwrap().await.unwrap();
    });
}

pub fn set_file(state: &crate::EngineState, id: usize, data: Arc<[u8]>) {
    let game = state.game.clone();
    let file = File {
        timestamp: Date::new_0(),
        mode: 33206,
        contents: Uint8Array::new_from_slice(&data),
    };
    leptos::task::spawn_local(async move {
        let (transaction, store, key, exists) = get_store(&game, id).await;
        let key: JsValue = key.into();
        if exists {
            // these confirmations are pointless currently but in the future the
            // engine frame will be cross-origin so then they will be purposeful
            if !window()
                .confirm_with_message(&format!(
                    "An attempt has been made to OVERWRITE the save in slot {id}. Proceed?"
                ))
                .unwrap_or_default()
            {
                transaction.abort().unwrap().await.unwrap();
                return;
            }
        }

        store
            .put(&serde_wasm_bindgen::to_value(&file).unwrap(), Some(&key))
            .unwrap()
            .await
            .unwrap();
        transaction.commit().unwrap().await.unwrap();
        location().reload().unwrap();
    });
}

pub fn delete_file(state: &crate::EngineState, id: usize) {
    let game = state.game.clone();
    leptos::task::spawn_local(async move {
        let (transaction, store, key, exists) = get_store(&game, id).await;
        let confirmed = || {
            window()
                .confirm_with_message(&format!(
                    "An attempt has been made to DELETE the save in slot {id}. Proceed?"
                ))
                .unwrap_or_default()
        };

        if !exists || !confirmed() {
            transaction.abort().unwrap().await.unwrap();
            return;
        }

        store
            .delete(idb::Query::Key(key.into()))
            .unwrap()
            .await
            .unwrap();
        transaction.commit().unwrap().await.unwrap();
        location().reload().unwrap();
    });
}

async fn get_store(game: &str, id: usize) -> (idb::Transaction, idb::ObjectStore, String, bool) {
    let factory = idb::Factory::new().unwrap();
    let database = factory
        .open(&format!("/easyrpg/{game}/Save"), None)
        .unwrap()
        .await
        .unwrap();
    let transaction = database
        .transaction(&["FILE_DATA"], idb::TransactionMode::ReadWrite)
        .unwrap();
    let store = transaction.object_store("FILE_DATA").unwrap();
    let key = format!("/easyrpg/{game}/Save/Save{id:>02}.lsd");
    let exists = store
        .count(Some(idb::Query::Key(JsValue::from_str(&key))))
        .unwrap()
        .await
        .unwrap()
        != 0;
    (transaction, store, key, exists)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct File {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub timestamp: Date,
    pub mode: u16,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub contents: Uint8Array,
}

impl From<File> for common::messages::play::SaveFile {
    fn from(value: File) -> Self {
        Self {
            contents: value.contents.to_vec().into(),
            timestamp: value.timestamp.to_iso_string().into(),
        }
    }
}
