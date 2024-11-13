use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Counter {
    value: Arc<Mutex<i32>>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn increment(&self) {
        let mut value = self.value.lock().await;
        *value += 1;
    }

    pub async fn get_value(&self) -> i32 {
        let value = self.value.lock().await;
        *value
    }
}