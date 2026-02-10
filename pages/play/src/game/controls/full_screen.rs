use leptos::prelude::*;

#[island]
pub fn FullScreen(children: Children) -> impl IntoView {
    let on_click = |_| {
        let document = document();

        if document.fullscreen_element().is_some() {
            document.exit_fullscreen();
        } else {
            // might be better to use a node_ref, but that requires moving the
            // <main id=layout> element into an island.
            let Some(layout) = document.get_element_by_id("layout") else {
                return;
            };

            drop(layout.request_fullscreen());
        }
    };

    view! {
        <button on:click=on_click>
            {children()}
        </button>
    }
}
