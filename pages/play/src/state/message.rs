use std::sync::Arc;

use leptos::prelude::*;

#[derive(Clone)]
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
                |id| id.into(),
            ),
            data,
            timestamp,
            filtered: None,
        }
    }
}

#[derive(Clone)]
pub enum MessageData {
    Map { author: Arc<str>, text: Arc<str> },
    Party { author: Arc<str>, text: Arc<str> },
    Global { author: Arc<str>, text: Arc<str> },
}

impl MessageData {
    pub fn author(&self) -> Option<Arc<str>> {
        match self {
            Self::Map { author, .. } => Some(author.clone()),
            Self::Party { author, .. } => Some(author.clone()),
            Self::Global { author, .. } => Some(author.clone()),
        }
    }
}
