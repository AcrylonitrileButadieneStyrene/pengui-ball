use leptos::prelude::*;

pub mod messages;

#[component]
pub fn EasyRPG(game: String, children: Children) -> impl IntoView {
    view! {
        <LoadPlayer game>
            <StartPlayer>{children()}</StartPlayer>
        </LoadPlayer>
    }
}

#[derive(Clone)]
struct Loaded(pub ReadSignal<bool>);

#[island]
fn LoadPlayer(game: String, children: Children) -> impl IntoView {
    let state = expect_context::<crate::EngineState>();
    messages::setup_handler(state);

    let (loaded, set_loaded) = signal(false);
    provide_context(Loaded(loaded));

    Effect::new(super::callbacks::setup);

    view! {
        <script src=format!("/yno/{game}/ynoengine-simd.js") onload=move || set_loaded(true) />
        {children()}
    }
}

#[island]
fn StartPlayer(children: Children) -> impl IntoView {
    let loaded = expect_context::<Loaded>();
    let state = expect_context::<crate::EngineState>();
    let ignore_next_blur = state.ignore_next_blur;
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
            crate::send(common::PlayMessage::EngineLoaded);
        });
    });

    let on_keydown = move |event: leptos::ev::KeyboardEvent| {
        if event.key() == "Tab" {
            ignore_next_blur.set(true);
            crate::send(common::PlayMessage::RegainFocus(event.shift_key()));
        }
    };

    view! {
        <canvas node_ref=node_ref id="canvas" tabindex=0 role="application" on:keydown=on_keydown />
        {children()}
    }
}
