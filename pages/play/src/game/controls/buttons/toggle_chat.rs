use leptos::prelude::*;

#[island]
pub fn ToggleChat(children: Children) -> impl IntoView {
    let state = crate::state();
    let game_config = state.config.game;
    let chat_hidden = move || game_config.get().chat_hidden;

    let on_click = move |_| game_config.update(|config| config.chat_hidden ^= true);

    view! {
        <button class="strickable" class:stricken=chat_hidden on:click=on_click>
            {children()}
        </button>
    }
}
