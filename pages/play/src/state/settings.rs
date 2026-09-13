use leptos::prelude::*;
use leptos_use::storage::{UseStorageOptions, use_local_storage_with_options};

pub struct State {
    pub global: RwSignal<GlobalSettings>,
    pub game: RwSignal<GameSettings>,
}

#[derive(Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GlobalSettings {}

#[derive(Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GameSettings {
    pub private_mode: u8,
    pub muted: bool,
    pub chat_hidden: bool,
}

impl State {
    pub fn new(game_id: &str) -> Self {
        Self {
            global: new_rw("settings"),
            game: new_rw(format!("settings_{game_id}")),
        }
    }
}

fn new_rw<T>(id: impl Into<Signal<String>>) -> RwSignal<T>
where
    T: Clone
        + Default
        + PartialEq
        + serde::Serialize
        + for<'a> serde::Deserialize<'a>
        + Send
        + Sync
        + 'static,
{
    let (get, set, _) = use_local_storage_with_options::<T, codee::string::JsonSerdeCodec>(
        id,
        UseStorageOptions::default().delay_during_hydration(true),
    );

    let rw = RwSignal::new(get.get_untracked());
    Effect::new(move || rw.set(get.get()));
    Effect::new(move || {
        let new = rw.get();
        if new != get.get_untracked() {
            set.set(new);
        }
    });
    rw
}
