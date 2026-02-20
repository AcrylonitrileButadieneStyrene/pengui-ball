use leptos::prelude::*;
use leptos_use::storage::{UseStorageOptions, use_local_storage_with_options};

pub struct State {
    pub global: RwSignal<GlobalConfig>,
    pub game: RwSignal<GameConfig>,
}

#[derive(Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GlobalConfig {}

#[derive(Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GameConfig {
    pub muted: bool,
}

impl State {
    pub fn new(game_id: &str) -> Self {
        Self {
            global: new_rw("config"),
            game: new_rw(format!("config_{game_id}")),
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
            set.set(new)
        };
    });
    rw
}
