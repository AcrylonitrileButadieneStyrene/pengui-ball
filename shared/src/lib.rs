#![allow(non_snake_case)]

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[cfg(feature = "hydrate")]
#[leptos::wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="stylesheet" href="/css/shared.css" />
                <HydrationScripts options islands=true />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();
    view! {
        <Router>
            <Routes fallback=|| ()>
                <Route path=path!("/") view=home::Home />
                <Route path=path!("/:game/") view=play::Play />
                <Route path=path!("/:game/engine") view=engine::Engine />
            </Routes>
        </Router>
    }
}
