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
                class:pop-out=true
                href="https://github.com/AcrylonitrileButadieneStyrene/pengui-ball"
                target="_blank"
            >
                <img src=format!(
                    "https://cdn.jsdelivr.net/gh/AcrylonitrileButadieneStyrene/yno-commumoji/assets/penguiBall{}.png",
                    match std::random::random::<u8>(..) % 4 + 1 {
                        1 => String::new(),
                        x => x.to_string(),
                    },
                ) />
            </a>

            <div class=style::middle />
            <Badges game=game.id.clone() />
            <CurrentUser />
        </header>
    }.into_any()
}

#[island]
fn Badges(game: std::sync::Arc<str>) -> impl IntoView {
    let state = crate::state();
    let badge = move || {
        state
            .api
            .user
            .read()
            .as_ref()
            .and_then(|user| user.as_ref().ok())
            .map(|user| {
                format!(
                    "https://ynoproject.net/{game}/images/badge/{}.png",
                    user.badge
                )
            })
    };

    let on_click = |_| {
        state.modal.set(Some(crate::modals::Modals::BadgeList));
    };

    view! {
        <button class=style::badge on:click=on_click>
            <img src=badge width=39 height=39 />
        </button>
    }
}

#[island]
fn CurrentUser() -> impl IntoView {
    let (once, set_once) = signal(true);
    let state = crate::state();

    let on_click = move |_| {
        let modal = state.api.user.map(|user| match user {
            Ok(user) if user.registered => crate::modals::Modals::LogOut,
            Ok(_) => crate::modals::Modals::LogIn,
            Err(_) => crate::modals::Modals::Cors,
        });
        state.modal.set(modal);
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

        content.map(|content| {
            view! {
                <button on:click=on_click class="button pop-out">
                    {content}
                </button>
            }
        })
    }
}
