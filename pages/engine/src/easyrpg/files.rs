use leptos::{
    wasm_bindgen::JsValue,
    web_sys::js_sys::{Reflect, Uint8Array},
};

pub fn get_file(state: &crate::EngineState, id: usize) {
    let game = state.game.clone();
    leptos::task::spawn_local(async move {
        let factory = indexed_db::Factory::<std::io::Error>::get().unwrap();
        let database = factory
            .open_latest_version(&format!("/easyrpg/{game}/Save"))
            .await
            .unwrap();
        // |event| async move {
        //         event
        //             .database()
        //             .build_object_store("FILE_DATA")
        //             .auto_increment()
        //             .create()?;
        //         Ok(())
        //     }
        database
            .transaction(&["FILE_DATA"])
            .rw()
            .run(move |transaction| async move {
                let object = transaction.object_store("FILE_DATA").unwrap();
                let data = object
                    .get(&JsValue::from_str(&format!(
                        "/easyrpg/{game}/Save/Save{id:>02}.lsd"
                    )))
                    .await
                    .unwrap()
                    .unwrap();
                let contents = Reflect::get(&data, &JsValue::from_str("contents")).unwrap();
                let contents = Uint8Array::from(contents).to_vec();

                Ok(())
            })
            .await
            .unwrap();
    });
}

pub fn set_file(state: &crate::EngineState, id: usize, data: Vec<u8>) {
    leptos::logging::log!("uploading save with {} bytes", data.len());
}

pub fn delete_file(state: &crate::EngineState, id: usize) {}
