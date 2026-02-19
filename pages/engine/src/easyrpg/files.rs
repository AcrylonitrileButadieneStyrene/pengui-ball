use std::sync::Arc;

use leptos::web_sys::js_sys::{Date, Uint8Array};

pub fn get_file(state: &crate::EngineState, id: usize) {
    let game = state.game.clone();
    leptos::task::spawn_local(async move {
        let (transaction, store, key) = get_store(&game, id).await;
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
        let (transaction, store, key) = get_store(&game, id).await;
        store
            .put(
                &serde_wasm_bindgen::to_value(&file).unwrap(),
                Some(&key.into()),
            )
            .unwrap()
            .await
            .unwrap();
        transaction.commit().unwrap().await.unwrap();
    });
}

pub fn delete_file(state: &crate::EngineState, id: usize) {
    let game = state.game.clone();
    leptos::task::spawn_local(async move {
        let (transaction, store, key) = get_store(&game, id).await;
        store
            .delete(idb::Query::Key(key.into()))
            .unwrap()
            .await
            .unwrap();
        transaction.commit().unwrap().await.unwrap();
    });
}

async fn get_store(game: &str, id: usize) -> (idb::Transaction, idb::ObjectStore, String) {
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

    (
        transaction,
        store,
        format!("/easyrpg/{game}/Save/Save{id:>02}.lsd"),
    )
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
