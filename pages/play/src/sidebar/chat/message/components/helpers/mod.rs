mod chat_message;

pub use chat_message::ChatMessageComponent;

pub fn timestamp(timestamp: chrono::DateTime<chrono::Local>) -> String {
    let format = if timestamp.date_naive() < chrono::Local::now().date_naive() {
        "%l:%M %p (%a)"
    } else {
        "%l:%M %p"
    };
    timestamp.format(format).to_string()
}
