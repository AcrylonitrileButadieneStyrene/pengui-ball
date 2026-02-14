use std::sync::Arc;

use common::messages::play::ConnectionStatus;
use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::{
    UseWebSocketOptions, UseWebSocketReturn, core::ConnectionReadyState, use_websocket_with_options,
};

mod command;
mod handler;
mod state;

pub use command::Command;
pub use state::State as SessionState;

use crate::CurrentGame;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Session() -> impl IntoView {
    let game = expect_context::<CurrentGame>();

    view! {
        <Connection game=game.id.clone()>
            <svg viewBox="0 0 18 18">
                <path d="m0 7q1.5-7 9-7 3 0 5.5 2.5l2-2.5 1.5 8h-8l2-2.5q-5-3.5-8 1.5h-4" />
                <path d="M18 11q-1.5 7-9 7-3 0-5.5-2.5l-2 2.5-1.5-8h8l-2 2.5q5 3.5 8-1.5h4" />
            </svg>
        </Connection>
    }
}

#[island]
fn Connection(game: Arc<str>, children: Children) -> impl IntoView {
    let state = crate::state();
    let room_status = state.engine.status;
    let session_status = state.session.status;
    let session_target = state.session.target;

    let (retry_count, set_retry_count) = signal(0);
    let (retry_handle, set_retry_handle) = signal(None::<TimeoutHandle>);

    let UseWebSocketReturn {
        ready_state,
        send,
        open,
        close,
        ..
    } = use_websocket_with_options::<String, String, FromToStringCodec, _, _>(
        &format!("wss://connect.ynoproject.net/{game}/session"),
        UseWebSocketOptions::default()
            .immediate(false)
            .reconnect_limit(leptos_use::ReconnectLimit::Limited(0))
            .on_message({
                let state = state.clone();
                move |message: &String| {
                    let parts = message.split('\u{FFFF}').collect::<Vec<_>>();
                    handler::on_message(&state, &parts);
                }
            }),
    );

    leptos::task::spawn(send_messages(state, Arc::new(send)));
    // manual reconnect and disconnect
    Effect::new({
        let open = open.clone();
        move || {
            close();
            if session_target.get().is_some() {
                open();
            }
        }
    });
    // auto reconnect
    Effect::new(move || {
        let ready_state = ready_state.get();
        session_status.set(ready_state);

        if let Some(handle) = retry_handle.get_untracked() {
            handle.clear();
        }

        if ready_state == ConnectionReadyState::Closed {
            if session_target.get().is_some() {
                let value = retry_count.get_untracked();
                set_retry_count(value + 1);

                set_retry_handle(
                    set_timeout_with_handle(
                        open.clone(),
                        std::time::Duration::from_secs(2_u64.pow(value.min(5))),
                    )
                    .ok(),
                );
            }
        } else if ready_state == ConnectionReadyState::Open {
            set_retry_count(0);
        }
    });

    view! {
        <button
            class=style::reconnect
            class:connected=move || ready_state.get() == ConnectionReadyState::Open
            class:connecting=move || ready_state.get() == ConnectionReadyState::Connecting
            class:room-connected=move || room_status.get() == ConnectionStatus::Connected
            class:room-connecting=move || room_status.get() == ConnectionStatus::Connecting
            on:click=move |_| state::State::connect_impl(session_target)
        >
            {children()}
        </button>
        <div>
            {move || match ready_state.get() {
                ConnectionReadyState::Open => "Connected",
                ConnectionReadyState::Connecting => "Connecting",
                ConnectionReadyState::Closing | ConnectionReadyState::Closed => "Disconnected",
            }}
        </div>
    }
}

async fn send_messages(
    state: Arc<crate::state::PlayState>,
    send: Arc<dyn Fn(&String) + Send + Sync>,
) {
    let receiver = state.session.channel.take_receiver().unwrap();
    while let Ok(message) = receiver.recv_async().await {
        let vec = match message {
            Command::Unknown(vec) => vec,
            Command::SayMap(msg) => vec!["say".to_string(), msg],
            Command::SayParty(msg) => vec!["psay".to_string(), msg],
            Command::SayGlobal(msg) => vec!["gsay".to_string(), msg],
        };

        send(&vec.join("\u{FFFF}"));
    }
}
