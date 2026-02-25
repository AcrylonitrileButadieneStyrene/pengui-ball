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
        &chrono::DateTime::parse_from_rfc3339(&save_file.timestamp)
            .unwrap()
            .with_timezone(&chrono::Local)
            .format("%Y-%m-%d-%Hh%Mm%Ss")
            .to_string(),
    ));
    link.click();
    link.remove();
}
