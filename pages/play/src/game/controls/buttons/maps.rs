use leptos::prelude::*;

use crate::game::controls::icon;

#[island]
pub fn Maps() -> impl IntoView {
    let state = crate::state();

    move || {
        state.locations.current_maps.read().as_ref().map(|maps| {
            maps.clone()
                .into_iter()
                .rev()
                .map(|map| {
                    let (wiki_link, description) = map.extract();

                    view! {
                        <a class="pop-out" href=wiki_link title=description target="yumeWikiMap">
                            <icon::Map />
                        </a>
                    }
                })
                .collect::<Vec<_>>()
        })
    }
}
