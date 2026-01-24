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
    let (once, set_once) = signal(true);
    let state = crate::state();

    move || {
        state.user.map(|user| match user {
            Ok(user) => Some(user.uuid.clone()),
            Err(_) => {
                if once.get_untracked() {
                    set_once(false);
                    state.modal.set(Some(crate::modals::Modals::CORS));
                }

                None
            }
        })
    }
}
