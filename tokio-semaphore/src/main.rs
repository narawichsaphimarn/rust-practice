use tokio::sync::{Semaphore, mpsc};
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    let semaphore = Arc::new(Semaphore::new(5)); // กำหนด concurrent task limit เป็น 5

    // Producer
    for i in 0..100 {
        let tx = tx.clone();
        let _semaphore = semaphore.clone();
        task::spawn(async move {
            let permit = _semaphore.acquire_owned().await;
            tx.send(i).await.unwrap();
            drop(permit); // ปล่อย semaphore ให้ task อื่นเข้าใช้งานได้
        });
    }

    // Consumer
    while let Some(value) = rx.recv().await {
        println!("Consumed: {}", value);
    }
}
