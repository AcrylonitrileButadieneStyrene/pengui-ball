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

            <div class=style::tabs>
                <tabs::Tab label="Chat".to_string() default=true>
                    <chat::Chat />
                </tabs::Tab>
                <tabs::Tab label="Players".to_string()>
                    <div>Under construction</div>
                </tabs::Tab>
                <tabs::Tab label="Parties".to_string()>
                    <div>Under construction</div>
                </tabs::Tab>
            </div>
        </div>
    }
    .into_any()
}
