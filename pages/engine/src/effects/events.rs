use leptos::prelude::*;

pub fn effect(state: crate::EngineState) {
    let timer = state.defocus_timeout;

    // i don't think this is necessary, but might as well have it
    window_event_listener(leptos::ev::focus, move |_| {
        if let Some(element) = state.easyrpg_player.canvas.get_untracked() {
            element.focus().unwrap();
        }
    });

    // disable easyrpg from seeing that the frame blurred, to add back in the
    // "bug" (feature) that keeps your inputs held down if you switch to chat.
    // the only annoyance with this is when your keys get stuck so maybe in the
    // future a fake blur event can be simulated after the window is refocused
    // (and after another input has been made) to free up the old keys
    window_event_listener(leptos::ev::blur, move |event| {
        // always propagate manually sent events
        if !event.is_trusted() {
            return;
        }

        event.stop_immediate_propagation();
        control_timer(timer, true);
    });
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
    } else {
        if let Some(handle) = defocus_timeout.get_untracked() {
            handle.clear();
        }
    }
}
