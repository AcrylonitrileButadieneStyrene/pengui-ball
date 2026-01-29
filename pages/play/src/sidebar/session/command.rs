pub enum Command {
    Unknown(Vec<String>),
}

pub struct Channel {
    sender: futures_channel::mpsc::Sender<Command>,
    receiver: std::sync::Mutex<Option<futures_channel::mpsc::Receiver<Command>>>,
}

impl std::ops::Deref for Channel {
    type Target = futures_channel::mpsc::Sender<Command>;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

impl Default for Channel {
    fn default() -> Self {
        let (sender, receiver) = futures_channel::mpsc::channel(16);

        Self {
            sender,
            receiver: std::sync::Mutex::new(Some(receiver)),
        }
    }
}

impl Channel {
    pub fn take_receiver(&self) -> Option<futures_channel::mpsc::Receiver<Command>> {
        self.receiver.lock().unwrap().take()
    }
}
