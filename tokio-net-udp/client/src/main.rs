use std::fmt::format;
use tokio::net::UdpSocket;
use std::io;
use winapi::shared::windef::POINT;
use winapi::um::winuser::GetCursorPos;
use std::time::{ Duration };
use std::thread::sleep;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?; // Bind to any available port
    let server_addr = "192.168.1.110:9876";
    socket.connect(server_addr).await?;
    loop {
        let m = get_cursor_point();
        let json = serde_json::to_string(&m)?;
        // let s = format!("X {} Y {}", m.x, m.y);
        // socket.send_to(s.as_bytes(), server_addr).await?;
        socket.send(json.as_bytes()).await?;
        println!("Message sent to {}", server_addr);

        // let mut buf = [0; 1024];
        // let (len, _) = socket.recv_from(&mut buf).await?;
        // println!("Received from server: {}", String::from_utf8_lossy(&buf[..len]));
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Mouse {
    x: i32,
    y: i32,
}

fn get_cursor_point() -> Mouse {
    let mut cursor_pos = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut cursor_pos);
    }
    Mouse { x: cursor_pos.x, y: cursor_pos.y }
}
