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
                <link rel="stylesheet" href="pkg/pengui-ball.css" />
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
    use crate::pages;

    leptos_meta::provide_meta_context();
    view! {
        <Router>
            <Routes fallback=pages::Error404>
                <Route path=path!("/") view=home::Home />
                <Route path=path!("/engine") view=engine::Engine />
                <Route path=path!("/:game") view=play::Play />
            </Routes>
        </Router>
    }
}
