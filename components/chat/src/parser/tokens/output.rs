use std::sync::Arc;

pub enum OutputToken {
    Text(String),
    Screenshot {
        url: String,
    },
    Emoji {
        id: String,
        url: Arc<str>,
        big: bool,
    },
}
