use leptos::prelude::*;

use crate::{
    sidebar::{location::Location, players::to_last_online},
    states::{locations::Location, players::friend::Friend},
};

stylance::import_style!(pub style, "friends.module.css");

#[island]
pub fn Friends() -> impl IntoView {
    let state = crate::state();
    let friends = state.players.friends;

    let header = move || {
        let (online, offline) = friends
            .read()
            .iter()
            .fold((0, 0), |(online, offline), friend| {
                // this isn't intentionally optimized to be branchless, it's
                // just more readable like this instead of using a branch.
                let state = friend.online as usize;
                (online + state, offline + 1 - state)
            });

        view! {
            <div class=style::header class:disabled=online == 0>
                {format!("Online - {online}")}
            </div>
            <div class=style::header class:disabled=offline == 0>
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
    let detail = if friend.online {
        let location = friend.map_id.parse().ok().map(|map| Location {
            game: friend.game.clone(),
            map,
            previous: friend.prev_map_id.as_ref().and_then(|str| str.parse().ok()),
            x: friend.x,
            y: friend.y,
        });
        view! { <Location location /> }.into_any()
    } else {
        view! { <span>{to_last_online(friend.last_active)}</span> }.into_any()
    };

    view! {
        <super::PlayerCell
            game=friend.game.clone()
            sprite=Some((friend.sprite_name.clone(), friend.sprite_index))
            name=friend.name.clone()
            detail
            medals=friend.medals.clone()
            badge=Some(friend.badge.clone())
            {..}
            style:order=(!friend.online).then_some("1")
        />
    }
}
