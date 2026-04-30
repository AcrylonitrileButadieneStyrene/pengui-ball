use leptos::prelude::*;

#[component]
pub fn Modal() -> impl IntoView {
    let config = expect_context::<std::sync::Arc<common::ServerConfiguration>>();
    let game = expect_context::<crate::CurrentGame>();
    let themes = config.themes.get(&game.id).cloned();

    view! {
        <super::Modal when=super::Modals::Themes>
            <div>{format!("{themes:#?}")}
            </div>
        </super::Modal>
    }
}
