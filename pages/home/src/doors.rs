use leptos::{ev, prelude::*, web_sys::HtmlAudioElement};

stylance::import_style!(pub style, "doors.module.css");

#[component]
pub fn Doors() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::Config>>();
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
    let selected = RwSignal::new(None::<usize>);
    provide_context(selected);

    window_event_listener(ev::pageshow, move |_| {
        selected.set(None);
    });

    view! { {children()} }
}

#[island]
fn DoorsSound() -> impl IntoView {
    let selected = expect_context::<RwSignal<Option<usize>>>();
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
