use leptos::prelude::*;

pub mod chat;
mod location;
mod player_count;
pub mod session;
mod tabs;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class=style::sidebar>
            <div class=style::connection>
                <session::Session />
                <player_count::PlayerCount />
            </div>

            <div class=style::location>
                <div>Location:</div>
                <location::CurrentLocation />
            </div>

            <tabs::Tabs>
                <chat::Chat />
            </tabs::Tabs>
        </div>
    }
    .into_any()
}
