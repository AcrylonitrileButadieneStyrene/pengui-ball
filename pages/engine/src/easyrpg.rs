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

// #[derive(Clone)]
// pub struct Player {
//     pub canvas: NodeRef<Canvas>,
//     pub object: ReadSignal<send_wrapper::SendWrapper<Option<PlayerJSObject>>>,
// }

#[island]
fn StartPlayer(children: Children) -> impl IntoView {
    let canvas = NodeRef::new();

    let loaded = use_context::<Loaded>().unwrap();
    Effect::new(move || {
        let loaded = loaded.0.get();
        if !loaded {
            return;
        }

        leptos::task::spawn_local(async move {
            let object = create_easyrpg_player().await;
            object.init_api();

            leptos::web_sys::js_sys::Reflect::set(
                &window(),
                &wasm_bindgen::JsValue::from_str("easyrpgPlayer"),
                &object,
            )
            .unwrap();
            // set_object(send_wrapper::SendWrapper::new(Some(object)));
        });
    });

    view! {
        <canvas id="canvas" node_ref=canvas tabindex=0 role="application" />
        {children()}
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type PlayerJSObject;

    #[wasm_bindgen(js_name = createEasyRpgPlayer)]
    pub async fn create_easyrpg_player() -> PlayerJSObject;

    #[wasm_bindgen(method, js_name = initApi)]
    pub fn init_api(this: &PlayerJSObject);
}
