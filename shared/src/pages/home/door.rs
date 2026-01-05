use leptos::prelude::*;
use web_sys::HtmlAudioElement;

stylance::import_style!(pub style, "door.module.css");

#[component]
pub fn Door(index: usize, game: crate::config::Game) -> impl IntoView {
    let aria_label = format!("Play {} online", &game.name);
    let logo_src = format!("https://ynoproject.net/images/logo_{}.png", &game.id);

    view! {
        <DoorSound id=game.id.clone() index>
            <DoorClickable id=game.id.clone() index=index>
                <div
                    class=style::door
                    style:--i=index.to_string()
                    aria-label=aria_label
                >
                    <DoorSpotlight index />
                    <DoorImage id=game.id.clone() index />
                    <img class=style::logo src=logo_src alt="" />
                    <img
                        class=style::shadow
                        src="https://ynoproject.net/images/door_shadow.png"
                        alt=""
                    />
                </div>
            </DoorClickable>
        </DoorSound>
    }
}

#[island]
fn DoorSound(id: String, index: usize, children: Children) -> impl IntoView {
    let node_ref = NodeRef::new();
    let selected = use_context::<ReadSignal<Option<usize>>>().unwrap();

    Effect::new(move || {
        if selected() == Some(index)
            && let Some(audio) = node_ref.get()
        {
            let audio = audio as HtmlAudioElement;
            set_timeout(
                move || drop(audio.play()),
                std::time::Duration::from_secs(1),
            );
        }
    });

    view! {
        <audio
            node_ref=node_ref
            src=format!("https://ynoproject.net/audio/door_{id}.wav")
            hidden=true
        />
        {children()}
    }
}

#[island]
fn DoorClickable(id: String, index: usize, children: Children) -> impl IntoView {
    let set_selected = use_context::<WriteSignal<Option<usize>>>().unwrap();

    let on_click = {
        let id = id.clone();
        move |e: web_sys::MouseEvent| {
            e.prevent_default();
            set_selected.set(Some(index));
            let id = id.clone();
            set_timeout(
                move || {
                    web_sys::js_sys::Reflect::set(
                        &window().document().unwrap(),
                        &web_sys::wasm_bindgen::JsValue::from_str("location"),
                        &web_sys::wasm_bindgen::JsValue::from_str(&id),
                    )
                    .unwrap();
                },
                std::time::Duration::from_millis(2500),
            );
        }
    };

    view! { <a href=id on:click=on_click>{children()}</a> }
}

#[island]
fn DoorSpotlight(index: usize) -> impl IntoView {
    let selected = use_context::<ReadSignal<Option<usize>>>().unwrap();

    view! {
        <Show when=move || selected() == Some(index)>
        <div class=style::spotlight />
        </Show>
    }
}

#[island]
fn DoorImage(id: String, index: usize) -> impl IntoView {
    let selected = use_context::<ReadSignal<Option<usize>>>().unwrap();

    view! {
        <img
            class=style::icon
            src=move || {
                format!(
                    "https://ynoproject.net/images/door_{}{id}.gif",
                    if selected() == Some(index) { "open_" } else { "" },
                )
            }
        />
    }
}
