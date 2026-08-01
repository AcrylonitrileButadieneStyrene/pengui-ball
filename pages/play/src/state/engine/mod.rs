use common::messages::play::ConnectionStatus;
use leptos::{html::Iframe, prelude::*};

mod save_timestamps;

pub use save_timestamps::SaveTimestamps;

pub struct State {
    pub frame: NodeRef<Iframe>,
    pub load_count: RwSignal<u32>,
    pub status: ReadSignal<ConnectionStatus>,
    pub save_timestamps: SaveTimestamps,

    set_status: WriteSignal<ConnectionStatus>,
}

impl Default for State {
    fn default() -> Self {
        let (status, set_status) = signal(ConnectionStatus::Disconnected);
        let frame = NodeRef::new();

        Self {
            frame,
            load_count: RwSignal::new(0),
            status,
            set_status,
            save_timestamps: SaveTimestamps::new(frame),
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

    pub fn set_status(&self, status: ConnectionStatus) {
        self.set_status.set(status);
    }
}
