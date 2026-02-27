use itertools::Itertools;
use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

use crate::{sidebar::session::Command, state, states::locations::LocationResolved};

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

    Effect::new({
        let state = state.clone();
        move || {
            if state.session.status.get() == ConnectionReadyState::Open {
                state.session.channel.send(Command::GetExpeds).unwrap();
            }
        }
    });

    Effect::new(move || {
        if !state.api.has_account.get() {
            return;
        }

        let Some(expeds) = expeds.get() else {
            return;
        };

        let locations = match state.locations.current_resolved.get() {
            Some(LocationResolved::Single { name, .. }) => vec![name],
            Some(LocationResolved::Multiple(locations)) => locations
                .iter()
                .map(|location| location.title.clone())
                .collect(),
            _ => return,
        };

        let mut locations = locations.into_iter().map(|location| {
            location
                .split_once(":")
                .map_or(location.clone(), |pair| pair.0.into())
        });

        let Some(exped) = locations.find_map(|location| {
            expeds
                .locations
                .iter()
                .filter(|exped| !exped.complete)
                .find(|exped| exped.title == location)
        }) else {
            return;
        };

        state
            .session
            .channel
            .send(Command::ClaimExpedLocation(
                exped.title.to_string(),
                exped.r#type == types::ExpedLocationType::Free,
            ))
            .unwrap();
    });

    move || {
        expeds.get().map(|expeds| {
            let locations = expeds
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

                    let views = locations
                        .map(|location| {
                            view! { <Location location=location.clone() /> }
                        })
                        .collect::<Vec<_>>();
                    view! {
                        <div class=style::header>{header}</div>
                        {views}
                    }
                })
                .collect::<Vec<_>>();
            let vms = expeds
                .vms
                .iter()
                .map(|vm| {
                    view! { <VM vm=vm.clone() /> }
                })
                .collect::<Vec<_>>();

            view! {
                {locations}
                <div class=style::header>Vending Machine Finder</div>
                {vms}
            }
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
            <Details ends_at experience complete />
        </div>
    }
}

#[component]
fn VM(vm: types::ExpedVM) -> impl IntoView {
    let types::ExpedVM {
        id,
        experience,
        ends_at,
        complete,
        ..
    } = vm;

    let node_ref = NodeRef::new();
    let image = LocalResource::new(move || async move {
        gloo_net::http::Request::get(&format!("api/vm?id={id}"))
            .send()
            .await
            .ok()?
            .binary()
            .await
            .ok()
    });

    Effect::new(move || {
        if let Some(response) = image.read().as_ref().flatten()
            && let Some(node) = node_ref.get()
        {
            let blob = gloo_file::Blob::new(&**response);
            let url = gloo_file::ObjectUrl::from(blob);
            let node: leptos::web_sys::HtmlImageElement = node;
            node.set_src(&url);
        }
    });

    view! {
        <div class=style::vm>
            <img node_ref=node_ref />
            <Details ends_at experience complete />
        </div>
    }
}

#[component]
fn Details(
    ends_at: chrono::DateTime<chrono::Local>,
    experience: u8,
    complete: bool,
) -> impl IntoView {
    view! {
        <div class=style::available>Available Until</div>
        <div class=style::ends_at>{ends_at.format("%-m/%d/%y, %-I:%M %p").to_string()}</div>
        <div class=style::experience>{format!("{experience} ExP")}</div>
        <div class=style::checkbox class:toggled=complete />
    }
}
