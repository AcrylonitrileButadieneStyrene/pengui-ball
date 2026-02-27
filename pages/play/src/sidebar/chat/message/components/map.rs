use std::sync::Arc;

pub struct MapMessage {
    pub author: Arc<str>,
}

impl super::ChatMessageComponent for MapMessage {
    fn author(&self) -> Arc<str> {
        self.author.clone()
    }
}
