mod observable;
mod observer;

use observable::Observable;
use observer::Observer;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use winapi::shared::windef::POINT;
use winapi::um::winuser::GetCursorPos;

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[tokio::main]
async fn main() {
    let (observable, rx) = Observable::new(Point { x: 0, y: 0 });

    let observer1 = Arc::new(Observer::new(1));
    let observer2 = Arc::new(Observer::new(2));

    observer1.clone().observe(rx.clone()).await;
    observer2.clone().observe(rx).await;

    tokio::spawn(loop_cursor(observable.clone()));
    tokio::signal::ctrl_c().await.unwrap();
}

async fn loop_cursor(observable: Observable) {
    loop {
        let y = get_cursor_point();
        let z = Point { x: y.x, y: y.y };
        observable.update_value(z);
        sleep(Duration::from_millis(50)).await;
    }
}

fn get_cursor_point() -> POINT {
    let mut cursor_pos = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut cursor_pos);
    }
    cursor_pos
}