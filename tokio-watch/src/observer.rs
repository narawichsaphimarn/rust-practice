use std::fmt::Debug;
use tokio::sync::watch::Receiver;
use tokio::task;
use std::sync::Arc;
use crate::Point;

pub struct Observer {
    id: u32,
}

impl Observer {
    pub fn new(id: u32) -> Self {
        Observer { id }
    }

    pub async fn observe(self: Arc<Self>, mut rx: Receiver<Point>) {
        task::spawn(async move {
            while rx.changed().await.is_ok() {
                let value = rx.borrow().clone();
                println!("Observer {} received new value: {:?}", self.id, value);
            }
        });
    }
}