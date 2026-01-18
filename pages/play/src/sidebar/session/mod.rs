use futures_util::{SinkExt as _, StreamExt as _};
use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::{UseWebSocketOptions, UseWebSocketReturn, use_websocket_with_options};

mod command;
mod handler;
mod reconnect;
mod status;

pub use command::{Command, CommandChannel};

stylance::import_style!(pub style, "mod.module.css");

#[allow(clippy::needless_pass_by_value)]
#[island]
pub fn Session(game: String) -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::state::State>>().unwrap();

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
    provide_context(ready_state);
    let reconnect = reconnect_handler(open, close);

    leptos::task::spawn(async move {
        let mut receiver = state.session_command.take_receiver().unwrap();
        while let Some(message) = receiver.next().await {
            let Command::Unknown(vec) = message;
            send(&vec.join("\u{FFFF}"));
        }
    });

    view! {
        <status::Status />
        <button
            class=style::reconnect
            on:click=move |_| {
                let mut reconnect = reconnect.clone();
                leptos::task::spawn(async move {
                    reconnect.send(()).await.unwrap();
                });
            }
        >
            R
        </button>
    }
}

// dumbest workaround
fn reconnect_handler(
    open: impl Fn() + Clone + Send + Sync + 'static,
    close: impl Fn() + Clone + Send + Sync + 'static,
) -> futures_channel::mpsc::Sender<()> {
    let (reconnect, mut __reconnect) = futures_channel::mpsc::channel::<()>(1);
    leptos::task::spawn(async move {
        while let Some(()) = __reconnect.next().await {
            close();
            open();
        }
    });
    reconnect
}

fn on_message(command: &str, args: &[&str]) {
    leptos::logging::log!("Received command {command} with arguments: {args:?}");
}
