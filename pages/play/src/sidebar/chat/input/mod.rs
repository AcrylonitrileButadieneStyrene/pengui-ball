use leptos::prelude::*;

use crate::states::players::player::PlayerStoreFields;

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
    let has_account = state.api.has_account;
    let guest_name = state.players.local.name();

    let is_unnamed = move || {
        let no_name = guest_name.get().is_none();
        let no_account = !has_account.get();
        no_name && no_account
    };

    view! { <div class:unnamed=is_unnamed>{children()}</div> }
}
