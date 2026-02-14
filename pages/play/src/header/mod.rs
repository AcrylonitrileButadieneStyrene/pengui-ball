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
                <img src="https://cdn.jsdelivr.net/gh/acrylonitrilebutadienestyrene/yno-commumoji/assets/penguiBall2.png" />
            </a>

            <div class=style::middle />
            <CurrentUser />
        </header>
    }
}

#[island]
fn CurrentUser() -> impl IntoView {
    let (once, set_once) = signal(true);
    let state = crate::state();

    let on_click = {
        let state = state.clone();
        move |_| {
            let modal = state.api.user.map(|user| match user {
                Some(user) if user.registered => crate::modals::Modals::LogOut,
                Some(_) => crate::modals::Modals::LogIn,
                None => crate::modals::Modals::Cors,
            });
            state.modal.set(modal);
        }
    };

    move || {
        let content = state.api.user.map(|user| match user {
            Some(user) if user.registered => "Log Out",
            Some(_) => "Log In",
            None => {
                if once.get_untracked() {
                    set_once(false);
                    state.modal.set(Some(crate::modals::Modals::Cors));
                }

                "Not Supported"
            }
        });

        let on_click = on_click.clone();
        content.map(|content| {
            view! {
                <button on:click=on_click class=style::button>
                    {content}
                </button>
            }
        })
    }
}
