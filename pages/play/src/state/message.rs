#[derive(Clone)]
pub struct Message {
    pub id: String,
    pub author: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
}

impl Message {
    pub fn new(id: Option<impl ToString>, author: impl ToString, content: impl ToString) -> Self {
        leptos_use::use_timestamp();

        let timestamp = chrono::Local::now();
        Self {
            id: id.map_or_else(
                || timestamp.timestamp_millis().to_string(),
                |id| id.to_string(),
            ),
            author: author.to_string(),
            content: content.to_string(),
            timestamp,
        }
    }
}
