use std::{
    collections::HashMap,
    sync::{Arc, nonpoison::Mutex},
};

use leptos::prelude::*;
use wasm_bindgen::JsValue;

type EmojiEntries = HashMap<Arc<str>, Arc<str>>;

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone)]
pub struct Emojis {
    #[wasm_bindgen(skip)]
    pub sources: Arc<Mutex<HashMap<Arc<str>, EmojiEntries>>>,
    #[wasm_bindgen(skip)]
    pub all: RwSignal<EmojiEntries>,
}

impl Emojis {
    pub fn new() -> Self {
        let value = Self {
            sources: Arc::new(Mutex::new(HashMap::new())),
            all: RwSignal::new(HashMap::default()),
        };

        if is_browser() {
            leptos::task::spawn_local({
                let value = value.clone();
                async move {
                    if let Ok(response) = gloo_net::http::Request::get("/yno/2kki/ynomoji.json")
                        .send()
                        .await
                        && let Ok(emojis) = response.json::<HashMap<Arc<str>, Arc<str>>>().await
                    {
                        let emojis = emojis
                            .into_iter()
                            .map(|(key, file)| {
                                (key, format!("/yno/2kki/images/ynomoji/{file}").into())
                            })
                            .collect();
                        value.add_source_inner("ynomoji".into(), emojis);
                    }
                }
            });
        }

        value
    }

    pub fn add_source_inner(&self, source: Arc<str>, emojis: HashMap<Arc<str>, Arc<str>>) {
        let mut sources = self.sources.lock();
        sources.insert(source, emojis);
        self.update(&sources);
        drop(sources);
    }

    fn update(&self, sources: &HashMap<Arc<str>, HashMap<Arc<str>, Arc<str>>>) {
        self.all.set(
            sources
                .values()
                .flat_map(|entries| entries.clone().into_iter())
                .collect(),
        );
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Emojis {
    #[wasm_bindgen::prelude::wasm_bindgen]
    pub fn add_source(
        &self,
        source: String,
        emojis: JsValue,
    ) -> Result<(), serde_wasm_bindgen::Error> {
        self.add_source_inner(source.into(), serde_wasm_bindgen::from_value(emojis)?);
        Ok(())
    }

    #[wasm_bindgen::prelude::wasm_bindgen]
    pub fn remove_source(&self, source: String) -> bool {
        let mut sources = self.sources.lock();
        let result = sources.remove(&Arc::from(source)).is_some();
        self.update(&sources);
        drop(sources);
        result
    }
}
