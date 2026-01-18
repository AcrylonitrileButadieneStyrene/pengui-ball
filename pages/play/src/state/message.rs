#[derive(Clone)]
pub struct Message {
    pub id: String,
    pub author: String,
    pub content: String,
}

impl Message {
    pub fn new(id: impl ToString, author: impl ToString, content: impl ToString) -> Self {
        Self {
            id: id.to_string(),
            author: author.to_string(),
            content: content.to_string(),
        }
    }
}
