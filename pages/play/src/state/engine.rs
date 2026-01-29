use common::messages::play::ConnectionStatus;
use leptos::{html::Iframe, prelude::*};

pub struct EngineState {
    pub frame: NodeRef<Iframe>,
    pub status: ReadSignal<ConnectionStatus>,

    set_status: WriteSignal<ConnectionStatus>,
}

impl Default for EngineState {
    fn default() -> Self {
        let (status, set_status) = signal(ConnectionStatus::Disconnected);

        Self {
            frame: NodeRef::new(),
            status,
            set_status,
        }
    }
}

impl EngineState {
    pub fn send(&self, message: common::EngineMessage) {
        Self::send_frame(self.frame, message);
    }

    pub fn send_frame(frame: NodeRef<Iframe>, message: common::EngineMessage) {
        if let Some(frame) = frame.get_untracked() {
            frame
                .content_window()
                .unwrap()
                .post_message(&message.ser(), "*")
                .unwrap();
        }
    }

    pub fn set_status(&self, status: ConnectionStatus) {
        self.set_status.set(status);
    }
}
