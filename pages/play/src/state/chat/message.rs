use std::sync::Arc;

use leptos::prelude::*;

#[derive(Clone)]
pub struct MessageItem {
    pub id: Arc<str>,
    pub text: Arc<str>,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub filtered: ReadSignal<bool>,
}

impl MessageItem {
    pub fn new(
        id: Option<impl Into<Arc<str>>>,
        text: Arc<str>,
        filtered: ReadSignal<bool>,
    ) -> Self {
        leptos_use::use_timestamp();

        let timestamp = chrono::Local::now();
        Self {
            id: id.map_or_else(
                || timestamp.timestamp_millis().to_string().into(),
                Into::into,
            ),
            text,
            timestamp,
            filtered,
        }
    }
}

pub trait MessageType: Send + Sync {
    fn render(&self, this: &MessageItem, state: &crate::state::PlayState) -> AnyView;
}
