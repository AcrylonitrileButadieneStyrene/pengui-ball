use futures_util::StreamExt as _;
use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::{UseWebSocketOptions, UseWebSocketReturn, use_websocket_with_options};

#[allow(clippy::needless_pass_by_value)]
#[island]
pub fn Session(game: String) -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::state::State>>().unwrap();
    let mut receiver = state
        .__session_command_queue
        .lock()
        .unwrap()
        .take()
        .unwrap();

    // DIFF: forest-orb increases the interval by 5 seconds on each attempt
    // i don't think that's too necessary so it's not done here.
    let UseWebSocketReturn { send, .. } =
        use_websocket_with_options::<String, String, FromToStringCodec, _, _>(
            &format!("wss://connect.ynoproject.net/{game}/session"),
            UseWebSocketOptions::default()
                .immediate(false)
                .reconnect_interval(5000)
                .reconnect_limit(leptos_use::ReconnectLimit::Infinite)
                .on_message(|message: &String| {
                    let mut parts = message.split('\u{FFFF}');
                    let Some(command) = parts.next() else {
                        return;
                    };
                    let args = parts.collect::<Vec<_>>();
                    on_message(command, &args);
                }),
        );

    leptos::task::spawn(async move {
        while let Some(message) = receiver.next().await {
            let crate::state::SessionCommand::Unknown(vec) = message;
            send(&vec.join("\u{FFFF}"));
        }
    });
}

fn on_message(command: &str, args: &[&str]) {
    leptos::logging::log!("Received command {command} with arguments: {args:?}");
}
