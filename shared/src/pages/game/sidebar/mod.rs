use leptos::prelude::*;

mod session;

#[component]
pub fn Sidebar() -> impl IntoView {
    
    view! {
        <div>Connected</div>
        <div>Location: Unknown Location</div>
        <div style="width: 100%; height: 100%; background-color: darkgreen;" />
    } 
}
