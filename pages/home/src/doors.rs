use leptos::{prelude::*, web_sys::HtmlAudioElement};

stylance::import_style!(pub style, "doors.module.css");

#[component]
pub fn Doors() -> impl IntoView {
    let config = use_context::<std::sync::Arc<common::Config>>().unwrap();
    let games = config.games.clone();

    view! {
        <main class=style::doors>
            <DoorsContext>
                <DoorsSound />
                <For
                    each=move || games.clone().into_iter().enumerate()
                    key=|game| game.clone()
                    let((index, game))
                >
                    <super::door::Door index game />
                </For>
            </DoorsContext>
        </main>
    }
}

#[island]
fn DoorsContext(children: Children) -> impl IntoView {
    let (selected, set_selected) = signal(None::<usize>);
    provide_context(selected);
    provide_context(set_selected);

    view! { {children()} }
}

#[island]
fn DoorsSound() -> impl IntoView {
    let selected = use_context::<ReadSignal<Option<usize>>>().unwrap();
    let node_ref = NodeRef::new();

    Effect::new(move || {
        if selected().is_some()
            && let Some(audio) = node_ref.get()
        {
            let audio = audio as HtmlAudioElement;
            drop(audio.play());
        }
    });

    view! {
        <audio
            node_ref=node_ref
            src="https://ynoproject.net/audio/door_effect.wav"
            hidden=true
            preload="none"
        />
    }
}
