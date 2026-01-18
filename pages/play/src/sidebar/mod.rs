use leptos::prelude::*;

mod chat;
mod player_count;
pub mod session;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class=style::connection>
            <session::Session />
            <player_count::PlayerCount />
        </div>

        <div>Location: Unknown Location</div>

        <chat::Chat />
    }
}
