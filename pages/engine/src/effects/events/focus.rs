use leptos::{html::Canvas, prelude::*};

pub fn effect(state: crate::EngineState) {
    let timer = state.defocus_timeout;
    let canvas = state.easyrpg_player.canvas;

    // for some reason if this is a closure it causes a wasm bindgen panic
    // that or it was because i was dispatching a focus event to the other one.
    window_event_listener(leptos::ev::touchstart, get_take_focus(canvas));
    // i don't think this is necessary, but might as well have it
    window_event_listener(leptos::ev::focus, get_take_focus(canvas));

    // disable easyrpg from seeing that the frame blurred, to add back in the
    // "bug" (feature) that keeps your inputs held down if you switch to chat.
    // the only annoyance with this is when your keys get stuck so maybe in the
    // future a fake blur event can be simulated after the window is refocused
    // (and after another input has been made) to free up the old keys
    window_event_listener(leptos::ev::blur, move |event| {
        crate::send(common::PlayMessage::FocusState(false));

        // always propagate manually sent events
        if !event.is_trusted() {
            return;
        }

        event.stop_immediate_propagation();
        control_timer(timer, true);
    });
}

fn get_take_focus<T>(canvas: NodeRef<Canvas>) -> impl Fn(T) {
    move |_event| {
        crate::send(common::PlayMessage::FocusState(true));
        if let Some(element) = canvas.get_untracked() {
            element.focus().unwrap();
        }
    }
}

pub fn control_timer(defocus_timeout: RwSignal<Option<TimeoutHandle>>, active: bool) {
    if active {
        defocus_timeout.set(
            set_timeout_with_handle(
                || {
                    let _ = window()
                        .dispatch_event(&leptos::web_sys::Event::new("blur").unwrap())
                        .unwrap();
                },
                std::time::Duration::from_millis(100),
            )
            .ok(),
        );
    } else if let Some(handle) = defocus_timeout.get_untracked() {
        handle.clear();
    }
}
