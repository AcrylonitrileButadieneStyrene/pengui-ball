pub enum Command {
    Unknown(Vec<String>),
}

pub struct CommandChannel {
    sender: futures_channel::mpsc::Sender<Command>,
    receiver: std::sync::Mutex<Option<futures_channel::mpsc::Receiver<Command>>>,
}

impl std::ops::Deref for CommandChannel {
    type Target = futures_channel::mpsc::Sender<Command>;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

impl CommandChannel {
    pub fn new() -> Self {
        let (sender, receiver) = futures_channel::mpsc::channel(16);

        Self {
            sender,
            receiver: std::sync::Mutex::new(Some(receiver)),
        }
    }

    pub fn take_receiver(&self) -> Option<futures_channel::mpsc::Receiver<Command>> {
        self.receiver.lock().unwrap().take()
    }
}
