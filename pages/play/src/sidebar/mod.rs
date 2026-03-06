use leptos::prelude::*;

use crate::components::{Tab, Tabs};

pub mod chat;
mod location;
mod player_count;
mod players;
pub mod session;

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

            <Tabs group="selected-sidebar-tab" large=true>
                <Tab label="Chat" default=true>
                    <chat::Chat />
                </Tab>
                <Tab label="Players">
                    <players::Players />
                </Tab>
                <Tab label="Parties">
                    <div>Under construction</div>
                </Tab>
            </Tabs>
        </div>
    }
    .into_any()
}
