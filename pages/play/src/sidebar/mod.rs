use leptos::prelude::*;

mod chat;
mod location;
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
                <location::CurrentLocation />
            </div>

            <chat::Chat />
        </div>
    }
    .into_any()
}
