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
        </header>
    }
}
