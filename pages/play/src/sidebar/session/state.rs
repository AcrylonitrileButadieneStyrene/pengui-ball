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

impl State {
    /// Only reconnects if already connected
    pub fn reconnect(&self) {
        self.target
            .update(|current| *current = current.and_then(|val| val.checked_add(1)));
    }

    pub fn connect_impl(target: RwSignal<Option<NonZeroUsize>>) {
        target.update(|current| {
            *current = Some(
                current
                    .and_then(|val| val.checked_add(1))
                    .unwrap_or(NonZeroUsize::new(1).unwrap()),
            );
        });
    }
}
