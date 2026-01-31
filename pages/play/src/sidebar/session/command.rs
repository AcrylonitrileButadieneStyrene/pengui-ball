pub enum Command {
    SayMap(String),
    Unknown(Vec<String>),
}

pub struct Channel {
    sender: flume::Sender<Command>,
    receiver: std::sync::Mutex<Option<flume::Receiver<Command>>>,
}

impl std::ops::Deref for Channel {
    type Target = flume::Sender<Command>;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

impl Default for Channel {
    fn default() -> Self {
        let (sender, receiver) = flume::bounded(16);

        Self {
            sender,
            receiver: std::sync::Mutex::new(Some(receiver)),
        }
    }
}

impl Channel {
    pub fn take_receiver(&self) -> Option<flume::Receiver<Command>> {
        self.receiver.lock().unwrap().take()
    }
}
