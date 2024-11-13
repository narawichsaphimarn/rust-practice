use std::sync::Arc;
use tokio::sync::{mpsc, watch, Mutex};
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (stop_tx, mut stop_rx) = mpsc::channel(5);
    let (watch_tx, mut watch_rx) = watch::channel(String::default());

    let handle = task::spawn(async move {
        loop {
            tokio::select! {
                Some(value) = stop_rx.recv() => {
                    if value == "STOP" {
                        println!("Received stop signal, stopping task...");
                        break;
                    } else {
                        println!("Received RUN signal, stopping task...");
                    }
                },
                _ = sleep(Duration::from_secs(1)) => {
                    println!("Task is running...");
                },
                _ = watch_rx.changed() => {
                    println!("Watching signal... {:?}", *watch_rx.borrow());
                }
            }
        }
    });

    sleep(Duration::from_secs(3)).await;
    stop_tx.send(String::from("RUN")).await.unwrap(); // ส่งสัญญาณหยุดไปยัง task

    sleep(Duration::from_secs(10)).await;
    watch_tx.send(String::from("STOP"));

    sleep(Duration::from_secs(10)).await;
    stop_tx.send(String::from("STOP")).await.unwrap(); // ส่งสัญญาณหยุดไปยัง task

    handle.await.unwrap();
}
