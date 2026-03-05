use leptos::prelude::*;

use crate::components::{Tab, Tabs};

#[component]
pub fn Players() -> impl IntoView {
    // create a new reactive scope because provide_context works differently
    // than i thought and overwrites the context of the parent (leptos moment)
    move || {
        view! {
            <Tabs group="selected-sidebar-players-tab">
                <Tab label="Map" default=true>
                    <div>Under construction</div>
                </Tab>
                <Tab label="Friends">
                    <div>Under construction</div>
                </Tab>
                <Tab label="Party">
                    <div>Under construction</div>
                </Tab>
                <Tab label="Enemies">
                    <div>Under construction</div>
                </Tab>
            </Tabs>
        }
    }
}
