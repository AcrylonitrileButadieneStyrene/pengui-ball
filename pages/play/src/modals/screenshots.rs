use leptos::prelude::*;

use crate::state::api::screenshots::Screenshot;

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

    view! { <Transition fallback=|| ()>{view}</Transition> }
}

fn screenshot() -> impl Fn(&Screenshot) -> AnyView {
    |screenshot| {
        view! {
            <img
                src=format!(
                    "https://connect.ynoproject.net/2kki/screenshots/{}/{}.png",
                    screenshot.uuid,
                    screenshot.id,
                )
                loading="lazy"
            />
        }
        .into_any()
    }
}
