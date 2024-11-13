use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;

struct Broadcast1 {
    channel: Receiver<&'static str>
}

impl Broadcast1 {
    async fn new(&mut self, num: i32) {
        if let Ok(message) = self.channel.recv().await {
            println!("Receiver {} got: {}", num, message);
        }
    }
}


#[tokio::main]
async fn main() {
    // สร้าง broadcast channel
    let (tx, mut rx1) = broadcast::channel(5);

    // subscribe receiver ใหม่
    let mut rx2 = tx.subscribe();  // Receiver ใหม่ (rx2)
    let mut rx3 = tx.subscribe();  // Receiver ใหม่ (rx3)

    let mut a = crate::Broadcast1{channel: rx1};
    crate::Broadcast1::new(&mut a, 1).await;
    let mut b = crate::Broadcast1{channel: rx2};
    crate::Broadcast1::new(&mut b, 2).await;
    let mut c = crate::Broadcast1{channel: rx3};
    crate::Broadcast1::new(&mut c, 3).await;

    // ส่งข้อมูลจาก sender
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send("Hello, broadcast!").unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

    }).await.unwrap();
}
