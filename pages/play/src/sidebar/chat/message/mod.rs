use leptos::prelude::*;

use crate::state::{Message, MessageData};

mod author;
mod icons;
pub mod location;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn MessageOuter(message: Message) -> impl IntoView {
    // not reactive
    let timestamp = message.timestamp.format(
        if message.timestamp.date_naive() < chrono::Local::now().date_naive() {
            "%l:%M %p (%a)"
        } else {
            "%l:%M %p"
        },
    );

    match message.data {
        MessageData::Map { author, text } => view! {
            <Message
                filtered=message.filtered
                header=move || {
                    view! { <span>{timestamp.to_string()}</span> }
                }
            >
                <author::Author uuid=author.clone() />
                <span>{text.to_string()}</span>
            </Message>
        }
        .into_any(),
        MessageData::Party { author, text } => view! {
            <Message
                filtered=message.filtered
                header=move || {
                    view! {
                        <span>{timestamp.to_string()}</span>
                    }
                }
            >
                <icons::People />
                <author::Author uuid=author.clone() />
                <span>{text.to_string()}</span>
            </Message>
        }
        .into_any(),
        MessageData::Global {
            author,
            text,
            location,
        } => view! {
            <Message
                filtered=message.filtered
                header=move || {
                    view! {
                        <location::Location location/>
                        <span>{timestamp.to_string()}</span>
                    }
                }
            >
                <icons::Megaphone />
                <author::Author uuid=author.clone() />
                <span>{text.to_string()}</span>
            </Message>
        }
        .into_any(),
        MessageData::Local { text } => view! {
            <Message
                filtered=message.filtered
                header=move || {
                    view! {
                        <span>Sending...</span>
                        <span>{timestamp.to_string()}</span>
                    }
                }
                {..}
                style:order="-1"
            >
                <span class=style::sending>{text}</span>
            </Message>
        }
        .into_any(),
    }
}

#[component]
fn Message(
    filtered: Option<ReadSignal<bool>>,
    #[prop(optional, into)] header: ViewFnOnce,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=style::message
            style:display=move || {
                if filtered.map_or_default(|filter| filter.get()) { "none" } else { "" }
            }
        >
            <div class=style::header>{header.run()}</div>
            <div>{children()}</div>
        </div>
    }
}
