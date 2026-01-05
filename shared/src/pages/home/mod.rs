use leptos::prelude::*;

mod door;
mod doors;

stylance::import_style!(pub style, "index.module.css");

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <leptos_meta::Title text="Yume Nikki Online Project" />
        <leptos_meta::Meta
            name="description"
            content="Play multiplayer Yume Nikki and Yume Nikki fangames for free! Ad-free and no registration required."
        />
        <img
            src="https://ynoproject.net/images/logo_yno.png"
            class=style::logo
            alt="Website logo"
        />
        <doors::Doors />
    }
}
