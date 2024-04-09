pub mod window;

fn main() {
    let window = window::Window::new(1920, 1080, None).unwrap();
    let _ = window.render_loop(||()).unwrap();
}
