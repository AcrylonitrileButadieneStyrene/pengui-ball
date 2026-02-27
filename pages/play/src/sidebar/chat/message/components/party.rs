use std::sync::Arc;

use leptos::prelude::*;

pub struct PartyMessage {
    pub author: Arc<str>,
}

impl super::ChatMessageComponent for PartyMessage {
    fn author(&self) -> Arc<str> {
        self.author.clone()
    }

    fn icon(&self) -> AnyView {
        view! { <super::icons::People /> }.into_any()
    }
}
