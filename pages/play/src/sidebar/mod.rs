use leptos::prelude::*;

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
        <div style="width: 100%; height: 100%; background-color: darkgreen;" />
    }
}
