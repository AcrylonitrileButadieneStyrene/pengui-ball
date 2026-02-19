use common::{EngineMessage, messages::play::ConnectionStatus};
use leptos::prelude::*;

use crate::state::engine;

stylance::import_style!(pub style, "saves.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::Saves>
            <div class=style::header>
                <div>Manage Save Data</div>
            </div>
            <div class=style::locked>
                <div>Please exit the game before uploading new saves.</div>
            </div>
            <div class=style::container>
                {(1..=15).map(|index| view! { <Slot index /> }).collect::<Vec<_>>()}
            </div>
        </super::Modal>
    }
}

#[component]
fn Slot(index: usize) -> impl IntoView {
    view! {
        <div class=style::slot>
            <div>File <span>{index}</span></div>
            <div>Empty</div>
            <Controls index />
        </div>
    }
}

#[island]
fn Controls(index: usize) -> impl IntoView {
    let state = crate::state();
    let frame = state.engine.frame;
    let input = NodeRef::new();

    let on_upload = move |_| {
        let input: leptos::web_sys::HtmlInputElement = input.get_untracked().unwrap();
        input.set_value("");
        input.click();
    };
    let on_download = move |_| engine::State::send_frame(frame, EngineMessage::GetSave(index));
    let on_delete = move |_| engine::State::send_frame(frame, EngineMessage::DeleteSave(index));

    let on_file = move |event| {
        let target = event_target::<leptos::web_sys::HtmlInputElement>(&event);

        if let Some(files) = target.files()
            && let Some(file) = files.get(0)
        {
            let path = std::path::PathBuf::from(file.name());
            let ext = path.extension().and_then(|ext| ext.to_str());
            if ext != Some("lsd") {
                window()
                    .alert_with_message("Please select a valid save file (.lsd format).")
                    .unwrap();
                // diff: forest-orb reclicks on the upload button
                return;
            }

            leptos::task::spawn_local(async move {
                let bytes = gloo_file::futures::read_as_bytes(&gloo_file::File::from(file))
                    .await
                    .unwrap();
                engine::State::send_frame(frame, EngineMessage::SetSave(index, bytes.into()))
            });
        }
    };

    view! {
        <input node_ref=input type="file" style:display="none" on:change=on_file />
        <div class=style::controls>
            <button on:click=on_upload prop:disabled=move || {
                state.engine.status.get() != ConnectionStatus::Disconnected
            }>
                <svg viewBox="0 0 18 18">
                    <path d="m12.75 18v-3.25h-2.25l3.75-4.25 3.75 4.25h-2.25v3.25h-3m-12.75-16.5q0-1.5 1.5-1.5h11.25l2.25 2.25v9.1m-2.25 5.15h-11.25q-1.5 0-1.5-1.5v-13.5m4.5-1.5v3.75q0 0.75 0.75 0.75h4.5q0.75 0 0.75-0.75v-3.75m-1.75 1v2.5h0.75v-2.5h-0.75m-5.75 15.5v-6.75q0-0.75 0.75-0.75h7.5q0.75 0 0.75 0.75v3.25m0 1.75v1.75m-7.5-6h6m-6 2.25h6m-6 2.25h6" />
                </svg>
            </button>
            <button on:click=on_download>
                <svg viewBox="0 0 18 18">
                    <path d="m12.75 10.5v3.75h-2.25l3.75 3.75 3.75-3.75h-2.25v-3.75h-3m-12.75-9q0-1.5 1.5-1.5h11.25l2.25 2.25v8.25m-2.25 6h-11.25q-1.5 0-1.5-1.5v-13.5m4.5-1.5v3.75q0 0.75 0.75 0.75h4.5q0.75 0 0.75-0.75v-3.75m-1.75 1v2.5h0.75v-2.5h-0.75m-5.75 15.5v-6.75q0-0.75 0.75-0.75h7.5q0.75 0 0.75 0.75v4.5m0 1.5v0.75m-7.5-6h6m-6 2.25h6m-6 2.25h6" />
                </svg>
            </button>
            <button on:click=on_delete>
                <svg viewBox="0 0 18 18">
                    <path d="m3.5 2h11q2 0 2 3h-15q0-3 2-3m4-2h2q2 0 2 2h-5q0-2 2-2m-5.5 5 1 13h10l1-13m-8.5 11-0.5-9m3 9v-9m2.5 9 0.5-9" />
                </svg>
            </button>
        </div>
    }
}
