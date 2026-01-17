use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::{
    UseWebSocketOptions, UseWebSocketReturn, core::ConnectionReadyState, use_websocket_with_options,
};

pub struct Session {
    pub status: Signal<ConnectionReadyState>,
    pub send: Box<dyn Fn(String, Vec<String>) + Send + Sync>,
}

#[island]
pub fn ProvideSession(game: String, children: Children) -> impl IntoView {
    // DIFF: forest-orb increases the interval by 5 seconds on each attempt
    // i don't think that's too necessary so it's not done here.
    let UseWebSocketReturn {
        ready_state, send, ..
    } = use_websocket_with_options::<String, String, FromToStringCodec, _, _>(
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

    provide_context(Session {
        status: ready_state,
        send: Box::new(move |command, mut arguments| {
            let mut vec = Vec::with_capacity(arguments.len() + 1);
            vec.push(command);
            vec.append(&mut arguments);
            let str = vec.join("\u{FFFF}");
            send(&str);
        }),
    });

    children();
}

fn on_message(command: &str, args: &[&str]) {
    leptos::logging::log!("Received command {command} with arguments: {args:?}");
}
