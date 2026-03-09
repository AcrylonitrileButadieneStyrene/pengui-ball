use leptos::prelude::*;

use crate::components::Scroller;

mod filters;
mod input;
pub mod message;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Chat() -> impl IntoView {
    view! {
        <div class=style::messages>
            <Scroller {..} style:flex-direction="column-reverse">
                <ChatMessages />
            </Scroller>
        </div>
        <input::ChatInput />
        <filters::Filters />
    }
    .into_any()
}

#[island]
pub fn ChatMessages() -> impl IntoView {
    let state = crate::state();
    let messages = state.chat.messages;
    let game_config = state.config.game;

    let each = move || messages.get().into_iter().rev().collect::<Vec<_>>();
    let messages = move || {
        let state = state.clone();
        view! {
            <For each=each key=|(id, _)| std::sync::Arc::as_ptr(id) let((_, (data, message)))>
                {message.render(&data, &state)}
            </For>
        }
    };

    let chat_visible = move || !game_config.read().chat_hidden;
    view! {
        <Show when=chat_visible fallback=|| ()>
            {messages()}
        </Show>
    }
}
