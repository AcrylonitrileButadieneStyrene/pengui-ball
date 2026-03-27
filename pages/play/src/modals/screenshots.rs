use leptos::prelude::*;

use crate::{state::api::screenshots::Screenshot, states::locations::Location};

stylance::import_style!(pub style, "screenshots.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::Screenshots>
            <div>My Screenshots</div>
            <Screenshots />
        </super::Modal>
    }
}

#[island]
fn Screenshots() -> impl IntoView {
    let state = crate::state();
    let screenshots = state.api.user_screenshots;

    let mut opened = false;
    Effect::new(move || {
        let was_opened = opened;
        opened = state.modal.get() == Some(super::Modals::Screenshots);
        if opened && !was_opened {
            screenshots.refetch();
        }
    });

    let view = move || {
        screenshots
            .read()
            .as_ref()
            .map(|screenshots| screenshots.iter().map(screenshot()).collect::<Vec<_>>())
    };

    view! {
        <Transition fallback=|| ()>
            <div class=style::container>{view}</div>
        </Transition>
    }
}

fn screenshot() -> impl Fn(&Screenshot) -> AnyView {
    |screenshot| {
        let location = Location {
            game: screenshot.game.clone(),
            map: screenshot.map.parse().unwrap(),
            x: screenshot.map_x,
            y: screenshot.map_y,
            previous: None,
        };

        view! {
            <div class=style::screenshot class=("pop-out", true)>
                <img
                    src=format!(
                        "https://connect.ynoproject.net/2kki/screenshots/{}/{}.png",
                        screenshot.uuid,
                        screenshot.id,
                    )
                    loading="lazy"
                />
                <div></div>
                <crate::sidebar::location::Location location />
            </div>
        }
        .into_any()
    }
}
