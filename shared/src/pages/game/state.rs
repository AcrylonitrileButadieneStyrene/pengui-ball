use leptos::prelude::*;

#[island]
pub fn Provider(children: Children) -> impl IntoView {
    provide_context(std::sync::Arc::new(State::new()));
    children()
}

pub struct State {
    pub send_session_command: futures_channel::mpsc::Sender<SessionCommand>,
    pub __session_command_queue:
        std::sync::Mutex<Option<futures_channel::mpsc::Receiver<SessionCommand>>>,
}

impl State {
    pub fn new() -> Self {
        let (send_session_command, session_command_queue) = futures_channel::mpsc::channel(16);

        Self {
            send_session_command,
            __session_command_queue: std::sync::Mutex::new(Some(session_command_queue)),
        }
    }
}

pub enum SessionCommand {
    Unknown(Vec<String>),
}
