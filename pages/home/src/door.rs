use std::sync::Arc;

use leptos::{
    prelude::*,
    web_sys::{self, HtmlAudioElement},
};

stylance::import_style!(pub style, "door.module.css");

#[component]
pub fn Door(index: usize, game: common::config::Game) -> impl IntoView {
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
                    <img class=style::logo src=logo_src alt="" height=60 />
                    <img
                        class=style::shadow
                        src="https://ynoproject.net/images/door_shadow.png"
                        alt=""
                        width=180
                        height=64
                    />
                </div>
            </DoorClickable>
        </DoorSound>
    }
}

#[island]
fn DoorSound(id: Arc<str>, index: usize, children: Children) -> impl IntoView {
    let node_ref = NodeRef::new();
    let selected = expect_context::<RwSignal<Option<usize>>>();

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
            prop:volume=0.5
            preload="none"
        />
        {children()}
    }
}

#[island]
fn DoorClickable(id: Arc<str>, index: usize, children: Children) -> impl IntoView {
    let selected = expect_context::<RwSignal<Option<usize>>>();

    let href = format!("/{id}/");
    let on_click = {
        let href = href.clone();
        move |e: web_sys::MouseEvent| {
            e.prevent_default();
            selected.set(Some(index));

            let href = href.clone();
            set_timeout(
                move || location().set_pathname(&href).unwrap(),
                std::time::Duration::from_millis(2500),
            );
        }
    };

    view! {
        <a href=href on:click=on_click>
            {children()}
        </a>
    }
}

#[island]
fn DoorSpotlight(index: usize) -> impl IntoView {
    let selected = expect_context::<RwSignal<Option<usize>>>();

    view! {
        <Show when=move || selected() == Some(index)>
            <div class=style::spotlight />
        </Show>
    }
}

#[island]
fn DoorImage(id: Arc<str>, index: usize) -> impl IntoView {
    let selected = expect_context::<RwSignal<Option<usize>>>();

    view! {
        <img
            class=style::icon
            src=move || {
                format!(
                    "https://ynoproject.net/images/door_{}{id}.gif",
                    if selected() == Some(index) { "open_" } else { "" },
                )
            }
            alt=""
            height=120
        />
    }
}
