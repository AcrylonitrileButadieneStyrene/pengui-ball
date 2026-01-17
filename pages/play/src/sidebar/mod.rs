use leptos::prelude::*;

mod session;

#[component]
pub fn Sidebar() -> impl IntoView {
    let game = use_context::<super::CurrentGame>().unwrap();

    view! {
        <session::Session game=game.id.clone() />
        <div>Location: Unknown Location</div>
        <div style="width: 100%; height: 100%; background-color: darkgreen;" />
    }
}
