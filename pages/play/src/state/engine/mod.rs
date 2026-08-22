use common::messages::play::ConnectionStatus;
use leptos::{html::Iframe, prelude::*};

mod save_timestamps;

pub use save_timestamps::SaveTimestamps;

pub struct State {
    pub frame: NodeRef<Iframe>,
    pub load_count: RwSignal<u32>,
    pub save_timestamps: SaveTimestamps,
    pub status: ReadSignal<ConnectionStatus>,
    set_status: WriteSignal<ConnectionStatus>,
    pub is_focused: ReadSignal<bool>,
    set_is_focused: WriteSignal<bool>,
}

impl Default for State {
    fn default() -> Self {
        let (status, set_status) = signal(ConnectionStatus::Disconnected);
        let (is_focused, set_is_focused) = signal(false);

        let frame = NodeRef::new();

        Self {
            frame,
            load_count: RwSignal::new(0),
            save_timestamps: SaveTimestamps::new(frame),
            status,
            set_status,
            is_focused,
            set_is_focused,
        }
    }
}

impl State {
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

    pub fn set_status(&self, value: ConnectionStatus) {
        self.set_status.set(value);
    }

    pub fn set_is_focused(&self, value: bool) {
        self.set_is_focused.set(value);
    }
}
