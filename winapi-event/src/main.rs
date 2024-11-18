use crate::utils::mouse::MouseUtil;
use crate::utils::window::Window;
use std::sync::Once;

mod utils;

static INIT: Once = Once::new();

pub async fn window_event() {
    unsafe {
        let hwmd = Window::create_window();
        Window::show_window(&hwmd);
        Window::show_cursor(false);
        let rect = Window::get_rect(&hwmd);
        Window::lock_cursor(&rect);
        Window::set_keyboard_hook();
        Window::event();
        // Window::destroy(&hwmd);
        Window::unset_keyboard_hook(&hwmd);
    }
}

pub async fn mouse() {
    loop {
        let point = MouseUtil::get_cursor_point();
        println!("mouse {:?}", point);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

pub async fn process() {}

#[tokio::main]
async fn main() {
    tokio::spawn(window_event());
    tokio::signal::ctrl_c().await.unwrap();
}
