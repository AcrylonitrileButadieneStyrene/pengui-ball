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

    let each = move || {
        messages
            .read()
            .iter()
            .rev()
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>()
    };
    let render = move |id: std::sync::Arc<str>| {
        messages.read().get(&id).map(|message| {
            view! { <message::MessageOuter message=message.clone() /> }
        })
    };

    view! { <For each=each key=|key| std::sync::Arc::as_ptr(key) children=render /> }
}
