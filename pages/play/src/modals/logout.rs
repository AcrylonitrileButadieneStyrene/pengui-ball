use leptos::{prelude::*, wasm_bindgen::prelude::JsValue, web_sys::js_sys::Reflect};

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::LogOut>
            <h2>Are you sure you want to log out?</h2>
            <LogoutButton {..} style:margin="auto" style:display="block" />
        </super::Modal>
    }
}

#[island]
fn LogoutButton() -> impl IntoView {
    let state = crate::state();
    let on_click = move |_| {
        Reflect::set(
            &document(),
            &JsValue::from_str("cookie"),
            &JsValue::from_str("auth=; max-age=0; path=/"),
        )
        .unwrap();

        state.user.refetch();
        state.modal.set(None);
        state.session.reconnect();
    };

    view! {
        <button on:click=on_click>Log Out</button>
    }
}
