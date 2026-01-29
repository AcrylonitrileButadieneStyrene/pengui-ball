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
pub struct Loaded(pub ReadSignal<bool>);

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
        });
    });

    view! {
        <canvas id="canvas" tabindex=0 role="application" />
        {children()}
    }
}
