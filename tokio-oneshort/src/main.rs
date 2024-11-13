use tokio::sync::oneshot;
//
// #[tokio::main]
// async fn main() {
//     // สร้าง oneshot channel
//     let (tx, rx) = oneshot::channel();
//
//     // ส่งข้อมูลแบบ one-time
//     tokio::spawn(async move {
//         tx.send("Hello, oneshot!").unwrap();
//     });
//
//     // รับข้อมูลที่ถูกส่งมา
//     match rx.await {
//         Ok(message) => println!("Received: {}", message),
//         Err(_) => println!("Sender dropped"),
//     }
// }


struct SendOnDrop {
    sender: Option<oneshot::Sender<&'static str>>,
}
impl Drop for SendOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            // Using `let _ =` to ignore send errors.
            let _ = sender.send("I got dropped!");
        }
    }
}

#[tokio::main]
async fn main() {
    let (send,mut recv) = oneshot::channel();

    let send_on_drop = SendOnDrop { sender: Some(send) };
    drop(send_on_drop);

    tokio::task::spawn(async move {
        loop {
            if let Ok(message) = &recv.await {
                println!("Received: {}", message);
            }
        }
    });
}
