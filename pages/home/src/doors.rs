use leptos::{ev, prelude::*, web_sys::HtmlAudioElement};

#[component]
pub fn Doors() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::ServerConfiguration>>();
    let games = config.games.clone();

    view! {
        <main class="doors">
            <DoorsContext>
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

    let audio_ref = NodeRef::new();
    Effect::new(move || {
        if selected().is_some()
            && let Some(audio) = audio_ref.get()
        {
            let audio = audio as HtmlAudioElement;
            drop(audio.play());
        }
    });

    view! {
        <audio
            node_ref=audio_ref
            src="https://ynoproject.net/audio/door_effect.wav"
            hidden=true
            preload="none"
        />
        { children() }
    }
}
