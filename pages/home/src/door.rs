use std::sync::Arc;

use leptos::{
    prelude::*,
    web_sys::{self, HtmlAudioElement},
};

#[component]
pub fn Door(index: usize, game: common::config::Game) -> impl IntoView {
    let aria_label = format!("Play {} online", &game.name);
    let logo_src = format!("https://ynoproject.net/images/logo_{}.png", &game.id);

    view! {
        <DoorWrapper id=game.id.clone() index>
            <div class="door" style=("--i", index.to_string()) aria-label=aria_label>
                <DoorSpotlight index />
                <DoorImage id=game.id.clone() index />
                <img class="logo" src=logo_src alt="" height=60 />
                <img
                    class="shadow"
                    src="https://ynoproject.net/images/door_shadow.png"
                    alt=""
                    width=180
                    height=64
                />
            </div>
        </DoorWrapper>
    }
}

#[island]
fn DoorWrapper(id: Arc<str>, index: usize, children: Children) -> impl IntoView {
    let audio_ref = NodeRef::new();
    let selected = expect_context::<RwSignal<Option<usize>>>();

    Effect::new(move || {
        if selected() == Some(index)
            && let Some(audio) = audio_ref.get()
        {
            let audio = audio as HtmlAudioElement;
            set_timeout(
                move || drop(audio.play()),
                std::time::Duration::from_secs(1),
            );
        }
    });

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
        <audio
            node_ref=audio_ref
            src=format!("https://ynoproject.net/audio/door_{id}.wav")
            hidden=true
            prop:volume=0.5
            preload="none"
        />
        <a href=href on:click=on_click>
            {children()}
        </a>
    }
}

#[island]
fn DoorSpotlight(index: usize) -> impl IntoView {
    expect_context::<RwSignal<Option<usize>>>()()
        .filter(|selected| *selected == index)
        .map(|_| view! { <div class="spotlight" /> });
}

#[island]
fn DoorImage(id: Arc<str>, index: usize) -> impl IntoView {
    let (open, set_open) = signal(false);
    let selected = expect_context::<RwSignal<Option<usize>>>();

    Effect::new(move || {
        if selected() == Some(index) {
            set_timeout(move || set_open(true), std::time::Duration::from_secs(1));
        } else {
            set_open(false);
        }
    });

    let src = move || {
        format!(
            "https://ynoproject.net/images/door_{}{id}.gif",
            if open() { "open_" } else { "" },
        )
    };

    view! { <img class="icon" src=src alt="" height=120 /> }
}
