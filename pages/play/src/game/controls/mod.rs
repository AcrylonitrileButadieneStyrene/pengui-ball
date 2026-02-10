use leptos::prelude::*;

mod full_screen;
mod icon;
mod mute;

stylance::import_style!(pub style, "mod.module.css");

#[component]
pub fn Controls() -> impl IntoView {
    view! {
        <div class=style::controls>
            <div>
                <icon::People />
                <icon::FloppyDisk />
                <icon::PaintPalette />
                <icon::SpeechBubble />
                <icon::SixLayerTerminal />
                <icon::Camera />
                <icon::Photograph />
                <icon::Gear />
                <mute::Mute>
                    <icon::Sound />
                </mute::Mute>
                <icon::LocationNeedle />
            </div>
            <div class=style::right>
                <full_screen::FullScreen>
                    <icon::Expand />
                </full_screen::FullScreen>
                <icon::Compass />
                <icon::FamilyTree />
                <icon::Map />
            </div>
        </div>
    }
}
