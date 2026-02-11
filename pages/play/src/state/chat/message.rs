use std::sync::Arc;

use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct Message {
    pub id: Arc<str>,
    pub data: MessageData,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub filtered: Option<ReadSignal<bool>>,
}

impl Message {
    pub fn new(id: Option<impl Into<Arc<str>>>, data: MessageData) -> Self {
        leptos_use::use_timestamp();

        let timestamp = chrono::Local::now();
        Self {
            id: id.map_or_else(
                || timestamp.timestamp_millis().to_string().into(),
                Into::into,
            ),
            data,
            timestamp,
            filtered: None,
        }
    }

    pub const fn text(&self) -> &Arc<str> {
        match &self.data {
            MessageData::Map { text, .. }
            | MessageData::Party { text, .. }
            | MessageData::Global { text, .. }
            | MessageData::Local { text } => text,
        }
    }
}

#[derive(Clone, Debug)]
pub enum MessageData {
    Map { author: Arc<str>, text: Arc<str> },
    Party { author: Arc<str>, text: Arc<str> },
    Global { author: Arc<str>, text: Arc<str> },
    Local { text: Arc<str> },
}
