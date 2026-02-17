use leptos::prelude::*;

mod filters;
mod input;
mod message;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Chat() -> impl IntoView {
    view! {
        <div class=style::messages>
            <div class=style::scroll_view>
                <ChatMessages />
            </div>
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

    let each = move || messages.get().into_iter().rev().collect::<Vec<_>>();

    view! {
        <For each=each key=|(id, _)| std::sync::Arc::as_ptr(id) let((_, message))>
            <message::MessageOuter message=message />
        </For>
    }
}
