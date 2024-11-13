use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};
use winapi::shared::windef::POINT;
use crate::Point;

#[derive(Clone)]
pub struct Observable {
    tx: Sender<Point>,
}

impl Observable {
    pub fn new(initial_value: Point) -> (Self, Receiver<Point>) {
        let (tx, rx) = watch::channel(initial_value);
        (Observable { tx }, rx)
    }

    pub fn update_value(&self, new_value: Point) {
        let _ = self.tx.send(new_value); // Broadcast new value to all observers
    }
}