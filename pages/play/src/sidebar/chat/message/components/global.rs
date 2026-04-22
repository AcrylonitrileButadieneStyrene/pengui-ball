use std::sync::Arc;

use leptos::prelude::*;

#[derive(Clone)]
pub struct GlobalMessage {
    pub author: Arc<str>,
    pub location: Option<locations::Location>,
}

impl super::ChatMessageComponent for GlobalMessage {
    fn author(&self) -> Arc<str> {
        self.author.clone()
    }

    fn header(&self) -> AnyView {
        let location = self.location.clone();
        view! { <crate::sidebar::location::Location location /> }.into_any()
    }

    fn icon(&self) -> AnyView {
        view! { <super::icons::Megaphone /> }.into_any()
    }
}
