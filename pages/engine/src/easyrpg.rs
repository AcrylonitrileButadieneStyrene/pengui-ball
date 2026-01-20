use leptos::prelude::*;

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
    let (loaded, set_loaded) = signal(false);
    provide_context(Loaded(loaded));

    Effect::new(super::callbacks::setup);

    view! {
        <script src=format!("yno/{game}/ynoengine-simd.js") onload=move || set_loaded(true) />
        {children()}
    }
}

#[island]
fn StartPlayer(children: Children) -> impl IntoView {
    let loaded = use_context::<Loaded>().unwrap();
    let state = use_context::<std::sync::Arc<crate::EngineState>>().unwrap();

    Effect::new(move || {
        if !loaded.0.get() {
            return;
        }

        let state = state.clone();
        leptos::task::spawn_local(async move { state.easyrpg_player.start().await });
    });

    view! {
        <canvas id="canvas" tabindex=0 role="application" />
        {children()}
    }
}
