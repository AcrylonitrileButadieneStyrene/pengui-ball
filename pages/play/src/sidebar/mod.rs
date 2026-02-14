use leptos::prelude::*;

mod chat;
mod player_count;
pub mod session;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class=style::chat>
            <div class=style::connection>
                <session::Session />
                <player_count::PlayerCount />
            </div>

            <div class=style::location>
                <div>Location:</div>
                <WithLocation />
            </div>

            <chat::Chat />
        </div>
    }
    .into_any()
}

#[island]
fn WithLocation() -> impl IntoView {
    let state = crate::state();
    view! {
        <chat::message::location::Location location=state.location />
    }
}
