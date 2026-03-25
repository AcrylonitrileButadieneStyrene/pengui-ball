use leptos::prelude::{location, window, window_event_listener};

pub fn effect() {
    window_event_listener(leptos::ev::error, |error| {
        let file = error.filename();
        if file.contains("ynoengine.wasm") || file.contains("ynoengine-simd.wasm") {
            window()
                .alert_with_message("Detected an EasyRPG crash. The game will now be restarted. Unsaved progress has been lost.")
                .unwrap();
            location().reload().unwrap();
        }
    });
}
