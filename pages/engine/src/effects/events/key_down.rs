use leptos::leptos_dom::helpers::window_event_listener;

pub fn effect() {
    #[allow(clippy::single_match)]
    window_event_listener(leptos::ev::keydown, |event| match &*event.code() {
        "KeyF" => crate::send(common::PlayMessage::OpenMap),
        _ => (),
    });
}
