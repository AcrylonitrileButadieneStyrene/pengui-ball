use leptos::prelude::*;

mod destination;
mod guest_name;
mod text_box;

#[component]
pub fn ChatInput() -> impl IntoView {
    view! {
        <Wrapper>
            <guest_name::GuestName />
            <destination::Destination />
            <text_box::TextBox />
        </Wrapper>
    }
}

#[island]
fn Wrapper(children: Children) -> impl IntoView {
    let state = crate::state();
    let user = state.api.user;
    let guest_name = state.chat.guest_name;

    let is_unnamed = move || {
        let no_name = guest_name.get().is_none();
        let no_account = user
            .read()
            .as_ref()
            .is_none_or(|response| response.as_ref().is_none_or(|user| !user.registered));
        no_name && no_account
    };

    view! { <div class:unnamed=is_unnamed>{children()}</div> }
}
