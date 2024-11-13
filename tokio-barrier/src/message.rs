use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct MessageChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

impl MessageChannel {
    pub fn new(buffer: usize) -> Self {
        let (tx, rx) = mpsc::channel(buffer);
        MessageChannel { tx, rx }
    }
}