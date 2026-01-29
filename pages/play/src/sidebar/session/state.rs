use std::num::NonZeroUsize;

use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

pub struct State {
    pub channel: super::command::Channel,
    pub status: RwSignal<ConnectionReadyState>,
    pub target: RwSignal<Option<NonZeroUsize>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            channel: super::command::Channel::default(),
            status: RwSignal::new(ConnectionReadyState::Closed),
            target: RwSignal::default(),
        }
    }
}
