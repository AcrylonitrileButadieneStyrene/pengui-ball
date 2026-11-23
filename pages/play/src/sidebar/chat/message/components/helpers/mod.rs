mod chat_message;

pub use chat_message::ChatMessageComponent;

pub fn timestamp(timestamp: jiff::Timestamp) -> String {
    let format = if timestamp < jiff::Timestamp::now() {
        "%l:%M %p (%a)"
    } else {
        "%l:%M %p"
    };

    jiff::fmt::strtime::format(format, timestamp).unwrap()
}
