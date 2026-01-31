use leptos::{prelude::*, web_sys::HtmlDivElement};

use crate::sidebar::session::Command;

stylance::import_style!(pub style, "input.module.css");

#[island]
pub fn ChatInput() -> impl IntoView {
    let state = crate::state();

    // let on_input = move |event| {
    //     let this = event_target::<HtmlDivElement>(&event);
    //     let Some(content) = this.text_content() else {
    //         return;
    //     };

    //     if content.len() <= 150 {
    //         return;
    //     }

    //     let selection = window().get_selection().unwrap().unwrap();
    //     let offset = selection.focus_offset();
    //     let end = content
    //         .char_indices()
    //         .map_while(|(index, _)| if index <= 150 { Some(index) } else { None })
    //         .last()
    //         .unwrap_or(content.len());
    //     this.set_text_content(Some(&content[0..end]));
    //     let range = document().create_range().unwrap();
    //     range
    //         .set_start(&this.first_child().unwrap(), offset.min(end as u32))
    //         .unwrap();
    //     selection.remove_all_ranges().unwrap();
    //     selection.add_range(&range).unwrap();
    // };

    let on_keypress = move |event: leptos::ev::KeyboardEvent| {
        if event.key() == "Enter" {
            event.prevent_default();
            let this = event_target::<HtmlDivElement>(&event);

            if let Some(content) = this.text_content()
                && !content.is_empty()
            {
                state
                    .session
                    .channel
                    .send(Command::SayMap(content))
                    .unwrap();
            }
            this.set_text_content(None);
        }
    };

    view! { <div contenteditable=true class=style::input on:keypress=on_keypress /> }
}
