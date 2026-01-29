use leptos::{
    prelude::*,
    wasm_bindgen::prelude::{Closure, JsValue},
    web_sys::js_sys::Reflect,
};

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::LogIn>
            {view! { <iframe src="https://ynoproject.net/\u{1F97A}" width=300 height=137 style:border="0" /> }
                .attr("loading", "lazy")}
            <LoginCallback/>
        </super::Modal>
    }
}

#[island]
fn LoginCallback() -> impl IntoView {
    Effect::new(move || {
        Reflect::set(
            &window(),
            &JsValue::from_str("onAuthCookieSet"),
            &Closure::<dyn Fn()>::new(on_auth_cookie_set).into_js_value(),
        )
        .unwrap();
    });
}

fn on_auth_cookie_set() {
    let state = crate::state();
    state.user.refetch();
    state.modal.set(None);
}
