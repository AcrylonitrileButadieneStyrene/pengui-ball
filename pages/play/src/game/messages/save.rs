use common::messages::play::SaveFile;
use leptos::{prelude::*, wasm_bindgen::JsCast as _};

pub fn data(state: &crate::state::PlayState, slot: usize, save_file: SaveFile) {
    let blob = gloo_file::Blob::new(&*save_file.contents);
    let url = gloo_file::ObjectUrl::from(blob);
    let link = document()
        .create_element("a")
        .unwrap()
        .dyn_into::<leptos::web_sys::HtmlAnchorElement>()
        .unwrap();
    link.set_href(&url);
    link.set_download(&format!(
        "{}_Save{slot:>02}_{}.lsd",
        state.locations.game,
        jiff::fmt::strtime::format(
            "%Y-%m-%d-%Hh%Mm%Ss",
            save_file.timestamp.parse::<jiff::Timestamp>().unwrap()
        )
        .unwrap()
    ));
    link.click();
    link.remove();
}
