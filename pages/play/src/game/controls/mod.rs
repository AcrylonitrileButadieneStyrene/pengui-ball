use leptos::prelude::*;

use crate::{
    game::controls::buttons::{
        full_screen::FullScreen, maps::Maps, modal::OpenModal, mute::Mute, private::Private,
    },
    modals::Modals,
};

mod buttons;
mod icon;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Controls() -> impl IntoView {
    view! {
        <div class=style::controls>
            <div>
                <Private>
                    <icon::People />
                </Private>
                <OpenModal modal=Modals::Saves>
                    <icon::FloppyDisk />
                </OpenModal>
                <icon::PaintPalette />
                <icon::SpeechBubble />
                <icon::SixLayerTerminal />
                <icon::Camera />
                <icon::Photograph />
                <icon::Gear />
                <Mute>
                    <icon::Sound />
                </Mute>
                <icon::LocationNeedle />
            </div>
            <div class=style::right>
                <FullScreen>
                    <icon::Expand />
                </FullScreen>
                <OpenModal modal=Modals::Expeds>
                    <icon::Compass />
                </OpenModal>
                <icon::FamilyTree />
                <Maps />
            </div>
        </div>
    }
}
