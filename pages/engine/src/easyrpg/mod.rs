use leptos::prelude::*;

mod callbacks;
pub mod files;
pub mod inputs;
pub mod messages;
pub mod state;

#[derive(Clone)]
struct Loaded(pub ReadSignal<bool>);

#[island]
pub fn LoadPlayer(children: Children) -> impl IntoView {
    let state = expect_context::<crate::EngineState>();
    messages::setup_handler(state);

    let (loaded, set_loaded) = signal(false);
    provide_context(Loaded(loaded));

    Effect::new(callbacks::setup);

    view! {
        <script src="_yno/ynoengine-simd.js" onload=move || set_loaded(true) />
        {children()}
    }
}

#[island]
pub fn StartPlayer() -> impl IntoView {
    let loaded = expect_context::<Loaded>();
    let state = expect_context::<crate::EngineState>();
    let node_ref = state.easyrpg_player.canvas;

    Effect::new(move || {
        if !loaded.0.get() {
            return;
        }

        leptos::task::spawn_local(async move {
            let config = crate::easyrpg::state::Configuration {
                websocket_url: format!("wss://api.ynoproject.net/{}/", state.game),
                game: state.game.clone(),
            };
            state.easyrpg_player.start(config).await;

            let handle =
                std::sync::Arc::new(std::sync::nonpoison::Mutex::new(None::<IntervalHandle>));
            *handle.lock() = Some(
                set_interval_with_handle(
                    {
                        let handle = handle.clone();
                        move || {
                            if let Some(Ok(_)) = state
                                .easyrpg_player
                                .call_untracked(|easyrpg| easyrpg.api().reset_canvas())
                            {
                                handle.lock().unwrap().clear();
                                crate::send(common::PlayMessage::EngineLoaded);
                            }
                        }
                    },
                    std::time::Duration::from_millis(100),
                )
                .unwrap(),
            );
        });
    });

    view! {
        <canvas
            node_ref=node_ref
            id="canvas"
            tabindex=0
            role="application"
            on:keydown=inputs::on_key_down
        />
    }
}
