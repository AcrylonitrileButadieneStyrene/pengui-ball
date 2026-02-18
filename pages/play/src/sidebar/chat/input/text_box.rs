use leptos::{prelude::*, wasm_bindgen::JsCast as _, web_sys::HtmlDivElement};

use crate::{
    sidebar::session::Command,
    state::{Message, MessageData, chat::MessageDestination},
};

stylance::import_style!(pub style, "text_box.module.css");

#[island]
pub fn TextBox() -> impl IntoView {
    let state = crate::state();
    let node_ref = state.chat.input;
    let frame = state.engine.frame;

    let on_input = move |event: leptos::ev::Event| {
        if event
            .dyn_ref::<leptos::ev::InputEvent>()
            .unwrap()
            .is_composing()
        {
            return;
        }

        clamp_input(&event);
    };

    let on_compositionend = |event: leptos::ev::CompositionEvent| {
        clamp_input(&event.dyn_into().unwrap());
    };

    let on_keypress = move |event: leptos::ev::KeyboardEvent| {
        if event.key() == "Enter" {
            event.prevent_default();
            let this = event_target::<HtmlDivElement>(&event);

            if let Some(content) = this.text_content()
                && !content.is_empty()
            {
                send(&state, content);
            }
            this.set_text_content(None);
        }
    };

    let on_keydown = move |event: leptos::ev::KeyboardEvent| {
        if event.key() == "Tab" {
            event.prevent_default();
            if let Some(frame) = frame.get_untracked() {
                drop(frame.focus());
            }
        }
    };

    view! {
        <div
            contenteditable=true
            node_ref=node_ref
            class=style::input
            on:input=on_input
            on:compositionend=on_compositionend
            on:keypress=on_keypress
            on:keydown=on_keydown
        />
    }
}

fn clamp_input(event: &leptos::ev::Event) {
    let target = event_target::<HtmlDivElement>(event);
    let Some(content) = target.text_content() else {
        return;
    };

    if content.len() <= 150 {
        return;
    }

    let selection = window().get_selection().unwrap().unwrap();
    let offset = selection.focus_offset() as usize;

    let caret = content
        .char_indices()
        .nth(offset)
        .map_or(content.len(), |(index, _)| index);
    let (left, mut right) = content.split_at(caret);

    if right.len() > 150 {
        right = clamp(right, 150);
    }

    let remaining = 150 - right.len();
    let left = clamp(left, remaining);

    target.set_text_content(Some(&[left, right].concat()));

    let range = document().create_range().unwrap();
    range
        .set_start(
            &target.first_child().unwrap(),
            u32::try_from(left.chars().count()).unwrap(),
        )
        .unwrap();
    selection.remove_all_ranges().unwrap();
    selection.add_range(&range).unwrap();
}

fn clamp(input: &str, length: usize) -> &str {
    let mut end = input.len().min(length);
    while !input.is_char_boundary(end) {
        end -= 1;
    }
    &input[..end]
}

fn send(state: &crate::State, content: String) {
    let message_data = MessageData::Sending {
        text: content.clone().into(),
    };

    let (command, filter) = match state.chat.destination.get_untracked() {
        MessageDestination::Map => (Command::SayMap(content), &state.chat.map.filter),
        MessageDestination::Party => (Command::SayParty(content), &state.chat.party.filter),
        MessageDestination::Global => (Command::SayGlobal(content), &state.chat.global.filter),
    };

    state.chat.sending.add(Message::new(
        None::<std::sync::Arc<str>>,
        message_data,
        filter.read_only(),
    ));
    state.session.channel.send(command).unwrap();
}
