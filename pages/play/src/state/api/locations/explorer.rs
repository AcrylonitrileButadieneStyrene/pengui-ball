use std::{
    collections::HashMap,
    fmt::Write as _,
    sync::{Arc, nonpoison::Mutex},
};

use leptos::prelude::*;

#[derive(Clone)]
pub enum Value {
    Pending(RwSignal<Option<Arc<Result<Arc<[Location]>, gloo_net::Error>>>>),
    Resolved(Arc<Result<Arc<[Location]>, gloo_net::Error>>),
}

pub type Container = Arc<Mutex<HashMap<(u16, Option<u16>), Value>>>;

#[derive(Debug, serde::Deserialize)]
pub struct Location {
    pub title: Arc<str>,
    // #[serde(rename = "titleJP", default)]
    // pub title_jp: Option<Arc<str>>,
}

pub fn fetch(explorer: Container, map: u16, previous: Option<u16>) {
    let pending = RwSignal::new(None);
    explorer
        .lock()
        .insert((map, previous), Value::Pending(pending));

    let mut endpoint = format!("/explorer/getMapLocationNames?mapId={map:>04}");
    if let Some(previous) = previous {
        write!(endpoint, "&prevMapId={previous:>04}").unwrap();
    }

    leptos::task::spawn_local(async move {
        let value = match gloo_net::http::Request::get(&endpoint).send().await {
            Ok(value) => value.json::<Arc<[Location]>>().await,
            Err(err) => Err(err),
        };
        let value = Arc::new(value);

        explorer
            .lock()
            .insert((map, previous), Value::Resolved(value.clone()));
        pending.set(Some(value));
    });
}
