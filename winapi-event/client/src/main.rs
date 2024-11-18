use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("Server listening on {}", socket.local_addr()?);

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received from {}: {}", addr, String::from_utf8_lossy(&buf[..len]));
        socket.send_to(b"Hello from server!", addr).await?;
    }
}
