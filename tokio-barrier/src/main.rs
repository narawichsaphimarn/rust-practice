mod counter;
mod message;
mod sync_barrier;

use counter::Counter;
use message::MessageChannel;
use sync_barrier::SyncBarrier;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // สร้าง counter
    let counter = Counter::new();
    let message_channel = MessageChannel::new(5);
    let sync_barrier = SyncBarrier::new(4);

    // สร้าง task ที่จะเพิ่มค่า counter
    let counter_clone = counter.clone();
    let task1 = tokio::spawn(async move {
        counter_clone.increment().await;
        println!("Counter incremented, value: {}", counter_clone.get_value().await);
    });

    // สร้าง task ที่จะส่งข้อความ
    let tx = message_channel.tx.clone();
    let task2 = tokio::spawn(async move {
        tx.send("Hello from task 2".to_string()).await.unwrap();
    });

    // สร้าง task ที่จะรับข้อความ
    let mut rx = message_channel.rx;
    let task3 = tokio::spawn(async move {
        if let Some(message) = rx.recv().await {
            println!("Received: {}", message);
        }

        // while let Some(message) = rx.recv().await {
        //     println!("Received: {}", message);
        // }
    });

    // สร้าง task ที่ใช้ barrier
    let barrier_clone = sync_barrier.barrier.clone();
    let task4 = tokio::spawn(async move {
        println!("Task 4 waiting at barrier...");
        barrier_clone.wait().await;
        println!("Task 4 passed the barrier");
    });

    let barrier_clone = sync_barrier.barrier.clone();
    let task5 = tokio::spawn(async move {
        println!("Task 5 waiting at barrier...");
        barrier_clone.wait().await;
        println!("Task 5 passed the barrier");
    });

    let barrier_clone = sync_barrier.barrier.clone();
    let task6 = tokio::spawn(async move {
        println!("Task 6 waiting at barrier...");
        barrier_clone.wait().await;
        println!("Task 6 passed the barrier");
    });

    let barrier_clone = sync_barrier.barrier.clone();
    let task7 = tokio::spawn(async move {
        println!("Task 7 waiting at barrier...");
        barrier_clone.wait().await;
        println!("Task 7 passed the barrier");
    });
    // รอทุก task ทำงานเสร็จ
    let _ = tokio::join!(task1, task2, task3, task4, task5, task6, task7);
}
