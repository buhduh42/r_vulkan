pub mod window;
pub mod event_loop;

pub fn init(window_width: u32, window_height: u32) {
    let event_loop = event_loop::EventLoop::new()?;
    window::Window::new(window_width, window_height);
}
