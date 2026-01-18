use leptos::prelude::*;

mod state;

pub use state::{ChatState, Message};

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Chat() -> impl IntoView {
    view! {
        <div class=style::container>
            <div class=style::scroller>
                <Messages />
            </div>
        </div>
    }
}

#[island]
pub fn Messages() -> impl IntoView {
    let state = use_context::<std::sync::Arc<crate::state::State>>().unwrap();
    let global = state.chat.global;

    view! {
        <For
            each=move || global.get()
            key=|item| item.id.clone()
            children=|message| view! { <Message message /> }
        />
    }
}

#[component]
pub fn Message(message: Message) -> impl IntoView {
    view! {
        <div>
            <span>{message.author}</span>
            {": "}
            <span>{message.content}</span>
        </div>
    }
}
