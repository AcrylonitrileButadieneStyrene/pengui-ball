use leptos::{html::Canvas, prelude::*};

pub fn on_key_down(event: leptos::ev::KeyboardEvent) {
    if event.key() == "Tab" {
        crate::send(common::PlayMessage::TakeFocus);
    }
}

pub fn press(canvas: NodeRef<Canvas>, keycode: u8, is_down: bool) {
    let Some(canvas) = canvas.get_untracked() else {
        return;
    };

    let code = match keycode {
        b'w' => "KeyW",
        b'a' => "KeyA",
        b's' => "KeyS",
        b'd' => "KeyD",
        b'z' => "KeyZ",
        b'x' => "KeyX",
        b'o' => "Key1",
        b'n' => "Key2",
        b't' => "Key3",
        b'f' => "Key4",
        b'i' => "Key5",
        b'r' => "Key6",
        b'g' => "Key7",
        b'e' => "Key8",
        b'k' => "Key9",
        _ => return,
    };

    let event = leptos::web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
        if is_down { "keydown" } else { "keyup" },
        &{
            let options = leptos::web_sys::KeyboardEventInit::new();
            options.set_code(code);
            options
        },
    )
    .unwrap();

    canvas.dispatch_event(&event).unwrap();
}
