use itertools::Itertools;
use leptos::prelude::*;

use crate::{
    sidebar::{location::Location, players::to_last_online},
    states::locations::Location,
};

#[island]
pub fn Friends() -> impl IntoView {
    let state = crate::state();
    let friends = state.players.friends;
    move || {
        let mut friends = friends
            .read()
            .iter()
            .chunk_by(|friend| friend.online)
            .into_iter()
            .map(|(online, friends)| {
                let header = if online { "Online" } else { "Offline" };
                let friends = friends
                    .map(|friend| {
                        let detail = if online {
                            let location = friend.map_id.parse().ok().map(|map| Location {
                                game: friend.game.clone(),
                                map,
                                previous: friend
                                    .prev_map_id
                                    .as_ref()
                                    .and_then(|str| str.parse().ok()),
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
                                sprite=(friend.sprite_name.clone(), friend.sprite_index)
                                name=friend.name.clone()
                                detail
                                medals=friend.medals.clone()
                                badge=Some(friend.badge.clone())
                            />
                        }
                    })
                    .collect::<Vec<_>>();

                view! {
                    <div>{format!("{header} - {}", friends.len())}</div>
                    {friends}
                }
            })
            .collect::<Vec<_>>();
        friends.reverse();
        friends
    }
}
