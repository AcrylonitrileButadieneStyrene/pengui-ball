use leptos::prelude::*;

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::LogOut>
            <h2>Are you sure you want to log out?</h2>
            <LogoutButton {..} style:margin="auto" style:display="block" />
        </super::Modal>
    }
}

#[island]
fn LogoutButton() -> impl IntoView {
    let state = crate::state();

    let on_click = move |_| {
        let state = state.clone();
        leptos::task::spawn_local(async move {
            gloo_net::http::Request::get("/api/seiko/logout")
                .send()
                .await
                .unwrap();
            state.user.refetch();
            state.modal.set(None);
            state.session.reconnect();
        });
    };

    view! {
        <button on:click=on_click>Log Out</button>
    }
}
