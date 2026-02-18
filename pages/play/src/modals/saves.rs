use leptos::prelude::*;

stylance::import_style!(pub style, "saves.module.css");

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <super::Modal when=super::Modals::Saves>
            <div class=style::header>
                <div>Manage Save Data</div>
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
            <div class=style::controls>
                <svg viewBox="0 0 18 18">
                    <path d="m12.75 18v-3.25h-2.25l3.75-4.25 3.75 4.25h-2.25v3.25h-3m-12.75-16.5q0-1.5 1.5-1.5h11.25l2.25 2.25v9.1m-2.25 5.15h-11.25q-1.5 0-1.5-1.5v-13.5m4.5-1.5v3.75q0 0.75 0.75 0.75h4.5q0.75 0 0.75-0.75v-3.75m-1.75 1v2.5h0.75v-2.5h-0.75m-5.75 15.5v-6.75q0-0.75 0.75-0.75h7.5q0.75 0 0.75 0.75v3.25m0 1.75v1.75m-7.5-6h6m-6 2.25h6m-6 2.25h6" />
                </svg>
                <svg viewBox="0 0 18 18">
                    <path d="m12.75 10.5v3.75h-2.25l3.75 3.75 3.75-3.75h-2.25v-3.75h-3m-12.75-9q0-1.5 1.5-1.5h11.25l2.25 2.25v8.25m-2.25 6h-11.25q-1.5 0-1.5-1.5v-13.5m4.5-1.5v3.75q0 0.75 0.75 0.75h4.5q0.75 0 0.75-0.75v-3.75m-1.75 1v2.5h0.75v-2.5h-0.75m-5.75 15.5v-6.75q0-0.75 0.75-0.75h7.5q0.75 0 0.75 0.75v4.5m0 1.5v0.75m-7.5-6h6m-6 2.25h6m-6 2.25h6" />
                </svg>
                <svg viewBox="0 0 18 18">
                    <path d="m3.5 2h11q2 0 2 3h-15q0-3 2-3m4-2h2q2 0 2 2h-5q0-2 2-2m-5.5 5 1 13h10l1-13m-8.5 11-0.5-9m3 9v-9m2.5 9 0.5-9" />
                </svg>
            </div>
        </div>
    }
}
