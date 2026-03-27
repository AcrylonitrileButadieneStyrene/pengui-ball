use leptos::prelude::*;

pub mod files;
pub mod messages;

#[derive(Clone)]
struct Loaded(pub ReadSignal<bool>);

#[island]
pub fn LoadPlayer(children: Children) -> impl IntoView {
    let state = expect_context::<crate::EngineState>();
    messages::setup_handler(state);

    let (loaded, set_loaded) = signal(false);
    provide_context(Loaded(loaded));

    Effect::new(super::callbacks::setup);

    view! {
        <script src=format!("_yno/ynoengine-simd.js") onload=move || set_loaded(true) />
        {children()}
    }
}

#[island]
pub fn StartPlayer(children: Children) -> impl IntoView {
    let loaded = expect_context::<Loaded>();
    let state = expect_context::<crate::EngineState>();
    let node_ref = state.easyrpg_player.canvas;

    Effect::new(move || {
        if !loaded.0.get() {
            return;
        }

        let state = state.clone();
        leptos::task::spawn_local(async move {
            let config = crate::state::easyrpg::Configuration {
                websocket_url: format!("wss://connect.ynoproject.net/{}/", state.game),
                game: state.game.clone(),
            };
            state.easyrpg_player.start(config).await;

            // waiting about 30ms seems to be necssary for some reason for the
            // api to work. an obvious delay isn't placed anywhere in forest-orb
            // so it's probably a side effect of its strange load system
            set_timeout(
                || crate::send(common::PlayMessage::EngineLoaded),
                std::time::Duration::from_millis(200),
            );
        });
    });

    let on_keydown = move |event: leptos::ev::KeyboardEvent| {
        if event.key() == "Tab" {
            crate::send(common::PlayMessage::RegainFocus(event.shift_key()));
        }
    };

    view! {
        <canvas node_ref=node_ref id="canvas" tabindex=0 role="application" on:keydown=on_keydown />
        {children()}
    }
}
