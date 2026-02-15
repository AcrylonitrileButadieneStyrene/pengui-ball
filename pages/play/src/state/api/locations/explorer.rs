use std::{
    collections::HashMap,
    fmt::Write as _,
    sync::{Arc, nonpoison::Mutex},
};

use leptos::{prelude::*, reactive::graph::ReactiveNode};

pub type Container =
    super::RwSignal<HashMap<(u16, Option<u16>), Arc<Mutex<Option<Arc<[Location]>>>>>>;

#[derive(serde::Deserialize)]
pub struct Location {
    pub title: Arc<str>,
    // #[serde(rename = "titleJP", default)]
    // pub title_jp: Option<Arc<str>>,
}

// this whole thing is so messed up and needs to be remade already
pub fn fetch(explorer: Container, map: u16, previous: Option<u16>) {
    let mutex = Arc::new(Mutex::new(None));
    explorer.update_untracked(|explorer| {
        explorer.insert((map, previous), mutex.clone());
    });

    let mut endpoint = format!("/explorer/getMapLocationNames?mapId={map:>04}");
    if let Some(previous) = previous {
        write!(endpoint, "&prevMapId={previous:>04}").unwrap();
    }

    leptos::task::spawn_local(async move {
        let value = match gloo_net::http::Request::get(&endpoint).send().await {
            Ok(value) => value.json::<Arc<[Location]>>().await,
            Err(err) => Err(err),
        };

        *mutex.lock() = value.ok();
        explorer.mark_dirty();
    });
}
