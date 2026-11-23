use std::sync::Arc;

use leptos::prelude::*;

#[derive(Clone)]
pub struct MessageItem {
    pub id: Arc<str>,
    pub text: Arc<str>,
    pub timestamp: jiff::Timestamp,
    pub visible: ReadSignal<bool>,
}

impl MessageItem {
    pub fn new(id: Option<impl Into<Arc<str>>>, text: Arc<str>, visible: ReadSignal<bool>) -> Self {
        let timestamp = jiff::Timestamp::now();
        Self {
            id: id.map_or_else(|| timestamp.as_millisecond().to_string().into(), Into::into),
            text,
            timestamp,
            visible,
        }
    }
}

pub trait MessageComponent: Send + Sync {
    fn render(&self, message: &MessageItem, state: &crate::state::PlayState) -> AnyView;
}
