use itertools::Itertools;
use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

use crate::{sidebar::session::Command, state};

pub mod types;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::Expeds>
            <div class=style::expeds_header>Expeditions</div>
            <Inner />
        </super::Modal>
        <svg viewBox="0 0 90 18" width=90>
            <defs>
                <path id="exped-star" d="m9 0.5 2 6.5h7l-5.5 4 2 6.5-5.5-4-5.5 4 2-6.5-5.5-4h7z" />
                <g id="exped-stars">
                    <use href="#exped-star" x="0" />
                    <use href="#exped-star" x="18" />
                    <use href="#exped-star" x="36" />
                    <use href="#exped-star" x="54" />
                    <use href="#exped-star" x="72" />
                </g>
            </defs>
        </svg>
    }
}

#[island]
fn Inner() -> impl IntoView {
    let state = state();
    let expeds = state.expeds;

    Effect::new(move || {
        if state.session.status.get() == ConnectionReadyState::Open {
            state.session.channel.send(Command::GetExpeds).unwrap();
        }
    });

    move || {
        expeds.get().map(|expeds| {
            expeds
                .locations
                .iter()
                .chunk_by(|location| location.r#type)
                .into_iter()
                .map(|(r#type, locations)| {
                    let header = match r#type {
                        types::ExpedLocationType::Free => "Free Expedition",
                        types::ExpedLocationType::Daily => "Daily",
                        types::ExpedLocationType::Weekly => "Weekly",
                        types::ExpedLocationType::Weekend => "Weekend",
                        types::ExpedLocationType::Special => "Special",
                    };

                    let view = locations
                        .map(|location| {
                            view! { <Location location=location.clone() /> }
                        })
                        .collect::<Vec<_>>();
                    view! {
                        <div class=style::location_header>{header}</div>
                        {view}
                    }
                })
                .collect::<Vec<_>>()
        })
    }
}

#[component]
fn Location(location: types::ExpedLocation) -> impl IntoView {
    let types::ExpedLocation {
        game,
        title,
        depth,
        experience,
        ends_at,
        complete,
        ..
    } = location;

    let depth = if &*game == "2kki" {
        depth * 2 / 3
    } else {
        depth
    };

    view! {
        <div class=style::location>
            <a href=format!("https://yume.wiki/{game}/{title}") target="yumeWiki">
                {title.clone()}
            </a>
            <svg class=style::stars viewBox="0 0 90 18" width=90>
                <use href="#exped-stars" fill="none" stroke="white" />
                <use
                    href="#exped-stars"
                    fill="white"
                    clip-path=format!("inset(0 {}px 0 0)", (10 - depth) * 9)
                />
            </svg>
            <div class=style::available>Available Until</div>
            <div class=style::ends_at>{ends_at.format("%-m/%d/%y, %-I:%M %p").to_string()}</div>
            <div class=style::experience>{format!("{experience} ExP")}</div>
            <div class=style::checkbox class:toggled=complete />
        </div>
    }
}
