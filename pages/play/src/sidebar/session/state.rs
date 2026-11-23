use std::num::NonZeroUsize;

use leptos::prelude::*;
use leptos_use_websocket::ConnectionReadyState;

pub struct State {
    pub channel: super::command::Channel,
    pub status: RwSignal<ConnectionReadyState>,
    pub is_connected: Signal<bool>,
    pub target: RwSignal<Option<NonZeroUsize>>,
}

impl Default for State {
    fn default() -> Self {
        let status = RwSignal::new(ConnectionReadyState::Closed);
        Self {
            channel: super::command::Channel::default(),
            status,
            is_connected: Signal::derive(move || {
                status.read() == leptos_use_websocket::ConnectionReadyState::Open
            }),
            target: RwSignal::new(
                #[cfg(debug_assertions)]
                None,
                #[cfg(not(debug_assertions))]
                NonZeroUsize::new(1),
            ),
        }
    }
}

impl State {
    /// Only reconnects if already connected
    pub fn reconnect(&self) {
        Self::reconnect_impl(self.target);
    }

    pub fn reconnect_impl(target: RwSignal<Option<NonZeroUsize>>) {
        target.update(|current| *current = current.and_then(|val| val.checked_add(1)));
    }

    pub fn connect(&self) {
        Self::connect_impl(self.target);
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
