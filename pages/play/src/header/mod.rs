use leptos::prelude::*;

mod logo;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Header() -> impl IntoView {
    let game = expect_context::<crate::CurrentGame>();

    view! {
        <header class=style::header>
            <logo::Logo />
            <img
                class=style::game_logo
                src=format!("https://ynoproject.net/images/logo_{}.png", game.id)
            />

            <div class=style::middle />
            <CurrentUser />
        </header>
    }
}

#[island]
fn CurrentUser() -> impl IntoView {
    let state = crate::state();

    move || {
        state.user.map(|user| match user {
            Ok(user) => user.uuid.clone().into_any(),
            Err(_) => {
                view! {
                    <crate::modals::Modal>
                        CORS issue. I will make a userscript for this later.<br/>
                        For now use a CORS bypass extension
                        (and configure it correctly for your own security).
                    </crate::modals::Modal>
                }
            }
            .into_any(),
        })
    }
}
