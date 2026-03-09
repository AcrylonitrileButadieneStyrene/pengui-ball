use std::sync::Arc;

use leptos::prelude::*;

use crate::components::{Tab, Tabs};

mod friends;
mod map;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Players() -> impl IntoView {
    // create a new reactive scope because provide_context works differently
    // than i thought and overwrites the context of the parent (leptos moment)
    players_inner
}

fn players_inner() -> impl IntoView {
    view! {
        <Tabs group="selected-sidebar-players-tab" class=style::players>
            <Tab label="Map" default=true>
                <map::Map />
            </Tab>
            <Tab label="Friends">
                <friends::Friends />
            </Tab>
            <Tab label="Party">
                <div>Under construction</div>
            </Tab>
            <Tab label="Enemies">
                <div>Under construction</div>
            </Tab>
        </Tabs>
    }
}

#[component]
fn PlayerCell(
    game: Arc<str>,
    sprite: Option<(Arc<str>, u8)>,
    name: Arc<str>,
    detail: AnyView,
    medals: [u8; 5],
    badge: Option<Arc<str>>,
) -> impl IntoView {
    let sprite = sprite.map_or_else(
        || view! { <div class=style::sprite /> }.into_any(),
        |sprite| {
            view! {
                <img
                    class=style::sprite
                    style:--sprite-index=sprite.1.to_string()
                    src=format!("https://ynoproject.net/data/{game}/CharSet/{}.png", sprite.0.replace('#', "%23"))
                />
            }
            .into_any()
        },
    );

    let badge = badge.map(|badge| {
        view! {
            <img
                class=style::badge
                loading="lazy"
                src=format!("https://ynoproject.net/2kki/images/badge/{badge}.png")
            />
        }
    });

    view! {
        <div class=style::row>
            {sprite} <span>{name}</span> <span class=style::detail>{detail}</span>
            <span>{medals}</span> {badge}
        </div>
    }
}

fn to_last_online(last_active: chrono::DateTime<chrono::Utc>) -> String {
    match (chrono::Utc::now() - last_active).num_minutes() {
        x if x < 0 => "A long time ago".to_string(),
        x if x == 1 => "A moment ago".to_string(),
        x if x < 60 => format!("{} minutes ago", x),
        x if x < 1440 => format!("{} hours ago", x / 60),
        x => format!("{} days ago", x / 1440),
    }
}
