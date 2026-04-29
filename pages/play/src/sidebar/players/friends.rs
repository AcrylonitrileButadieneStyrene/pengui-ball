use leptos::prelude::*;

use crate::{
    sidebar::{location::Location, players::to_last_online},
    states::players::friend::Friend,
};

stylance::import_style!(pub style, "friends.module.css");

#[island]
pub fn Friends() -> impl IntoView {
    let state = crate::state();
    let friends = state.players.friends;

    let header = move || {
        let (online, offline, pending) = friends.read().iter().fold(
            (0, 0, 0),
            |(mut online, mut offline, mut pending), friend| {
                let which = match (friend.accepted, friend.online) {
                    (false, _) => &mut pending,
                    (true, true) => &mut online,
                    (true, false) => &mut offline,
                };
                *which += 1;

                (online, offline, pending)
            },
        );

        view! {
            <div class=style::header class:disabled=pending == 0>
                {format!("Pending - {pending}")}
            </div>
            <div class=style::header class:disabled=online == 0 style:order="1">
                {format!("Online - {online}")}
            </div>
            <div class=style::header class:disabled=offline == 0 style:order="3">
                {format!("Offline - {offline}")}
            </div>
        }
    };

    view! {
        {header}
        <For each=friends key=|friend| friend.clone() let(friend)>
            <Friend friend />
        </For>
    }
}

#[component]
fn Friend(friend: Friend) -> impl IntoView {
    let add_friend = {
        let uuid = friend.uuid.clone();
        move || {
            let req = gloo_net::http::Request::get(&format!(
                "https://api.ynoproject.net/2kki/api/addplayerfriend?uuid={}",
                &uuid
            ));
            leptos::task::spawn_local(async {
                req.send().await.unwrap();
            });
        }
    };

    let remove_friend = {
        let uuid = friend.uuid.clone();
        move || {
            let req = gloo_net::http::Request::get(&format!(
                "https://api.ynoproject.net/2kki/api/removeplayerfriend?uuid={}",
                &uuid
            ));
            leptos::task::spawn_local(async {
                req.send().await.unwrap();
            });
        }
    };

    let detail = match (friend.accepted, friend.incoming, friend.online) {
        (true, _, true) => {
            let location = friend.map_id.parse().ok().map(|map| locations::Location {
                game: friend.game.clone(),
                map,
                previous: friend.prev_map_id.as_ref().and_then(|str| str.parse().ok()),
                x: friend.x,
                y: friend.y,
            });
            view! { <Location location /> }.into_any()
        }
        (true, _, false) => view! { <span>{to_last_online(friend.last_active)}</span> }.into_any(),
        (false, true, _) => view! {
            <span>
                <button class=style::action class:pop-up=true on:click=move |_| add_friend()>
                    "Accept"
                </button>
                <button class=style::action class:pop-up=true on:click=move |_| remove_friend()>
                    "Reject"
                </button>
            </span>
        }
        .into_any(),
        (false, false, _) => view! {
            <button class=style::action class:pop-up=true on:click=move |_| remove_friend()>
                "Cancel"
            </button>
        }
        .into_any(),
    };

    view! {
        <super::PlayerCell
            game=friend.game.clone()
            sprite=Some((friend.sprite_name.clone(), friend.sprite_index))
            name=friend.name.clone()
            detail
            medals=friend.medals
            badge=Some(friend.badge.clone())
            {..}
            style:order=if friend.online { "2" } else { "4" }
        />
    }
}
