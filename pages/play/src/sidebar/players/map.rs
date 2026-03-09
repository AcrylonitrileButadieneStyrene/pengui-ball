use leptos::prelude::*;
use reactive_stores::Store;

use crate::states::players::player::{Player, PlayerStoreFields as _};

#[island]
pub fn Map() -> impl IntoView {
    let state = crate::state();

    view! {
        <For each=state.players.in_map key=|(id, _)| *id let((_, player))>
            <Player state=state.clone() player />
        </For>
    }
}

#[component]
pub fn Player(state: crate::State, player: Store<Player>) -> impl IntoView {
    move || {
        view! {
            <super::PlayerCell
                game=state.locations.game.clone()
                sprite=player.sprite().get()
                name=player.name().get().unwrap_or_else(|| "<Unnamed Player>".into())
                detail=().into_any()
                medals=player.medals().get()
                badge=player.badge().get()
            />
        }
    }
}
