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
            <a
                class=style::project_logo
                role="button"
                href="https://github.com/AcrylonitrileButadieneStyrene/pengui-ball"
                target="_blank"
            >
                <img src=format!(
                    "https://cdn.jsdelivr.net/gh/AcrylonitrileButadieneStyrene/yno-commumoji/assets/penguiBall{}.png",
                    match std::random::random::<u8>(..) % 3 + 1 {
                        1 => String::new(),
                        x => x.to_string(),
                    },
                ) />
            </a>

            <div class=style::middle />
            <CurrentUser />
        </header>
    }.into_any()
}

#[island]
fn CurrentUser() -> impl IntoView {
    let (once, set_once) = signal(true);
    let state = crate::state();

    let on_click = {
        let state = state.clone();
        move |_| {
            let modal = state.api.user.map(|user| match user {
                Ok(user) if user.registered => crate::modals::Modals::LogOut,
                Ok(_) => crate::modals::Modals::LogIn,
                Err(_) => crate::modals::Modals::Cors,
            });
            state.modal.set(modal);
        }
    };

    move || {
        let content = state.api.user.map(|user| match user {
            Ok(user) if user.registered => "Log Out",
            Ok(_) => "Log In",
            Err(_) => {
                if once.get_untracked() {
                    set_once(false);
                    state.modal.set(Some(crate::modals::Modals::Cors));
                }

                "Not Supported"
            }
        });

        let on_click = on_click.clone();
        content.map(|content| {
            view! { <button on:click=on_click>{content}</button> }
        })
    }
}
