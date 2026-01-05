use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn Game() -> impl IntoView {
    let params = use_params_map();
    let game = move || params.with(|params| params.get("game").unwrap_or_default());

    let (count, set_count) = signal(0);
    let on_click = move |_| *set_count.write() += 1;

    view! {
        // <A href="/test2">test2</A>
        <p>The game is {game}</p>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
