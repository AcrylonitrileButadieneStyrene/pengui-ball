use leptos::prelude::*;

use crate::state::Message;

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
    }
}

#[island]
pub fn ChatMessages() -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::state::State>>().unwrap();
    let global = state.chat.global;

    let render = move |message: Message| {
        let author = state
            .players
            .with_untracked(|players| players.get(&message.author).cloned());
        view! { <message::ChatMessage message author /> }
    };

    view! { <For each=move || global.get() key=|item| item.id.clone() children=render /> }
}
