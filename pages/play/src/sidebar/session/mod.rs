use common::messages::play::ConnectionStatus;
use futures_util::{SinkExt as _, StreamExt as _};
use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::{
    UseWebSocketOptions, UseWebSocketReturn, core::ConnectionReadyState, use_websocket_with_options,
};

mod command;
mod handler;

pub use command::{Command, CommandChannel};

use crate::CurrentGame;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Session() -> impl IntoView {
    let game = use_context::<CurrentGame>().unwrap();

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
fn Connection(game: String, children: Children) -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::state::State>>().unwrap();
    let room_state = state.engine.status.clone();

    // DIFF: forest-orb increases the interval by 5 seconds on each attempt
    // i don't think that's too necessary so it's not done here.
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
            .reconnect_interval(5000)
            .reconnect_limit(leptos_use::ReconnectLimit::Infinite)
            .on_message({
                let state = state.clone();
                move |message: &String| {
                    let parts = message.split('\u{FFFF}').collect::<Vec<_>>();
                    handler::on_message(&state, &parts);
                }
            }),
    );
    let reconnect = reconnect_handler(open, close);

    Effect::new({
        let state = state.clone();
        move || {
            if ready_state.get() != ConnectionReadyState::Open {
                return;
            }

            state.engine.send(common::EngineMessage::Connect);
        }
    });

    leptos::task::spawn(async move {
        let mut receiver = state.session_command.take_receiver().unwrap();
        while let Some(message) = receiver.next().await {
            let Command::Unknown(vec) = message;
            send(&vec.join("\u{FFFF}"));
        }
    });

    view! {
        <button
            class=style::reconnect
            class:connected=move || ready_state.get() == ConnectionReadyState::Open
            class:connecting=move || ready_state.get() == ConnectionReadyState::Connecting
            class:room-connected=move || room_state.get() == ConnectionStatus::Connected
            class:room-connecting=move || room_state.get() == ConnectionStatus::Connecting
            on:click=move |_| {
                let mut reconnect = reconnect.clone();
                leptos::task::spawn(async move {
                    reconnect.send(()).await.unwrap();
                });
            }
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

// dumbest workaround
fn reconnect_handler(
    open: impl Fn() + Clone + Send + Sync + 'static,
    close: impl Fn() + Clone + Send + Sync + 'static,
) -> futures_channel::mpsc::Sender<()> {
    let (reconnect, mut __reconnect) = futures_channel::mpsc::channel::<()>(1);
    leptos::task::spawn(async move {
        while __reconnect.next().await == Some(()) {
            close();
            open();
        }
    });
    reconnect
}
