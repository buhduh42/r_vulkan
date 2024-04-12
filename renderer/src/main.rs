pub mod window;
pub mod vulkan;

use window::Window;

use self::vulkan::Vulkan;

fn main() {
    //TODO these shouldn't be called (Window|Vulkan)::new()
    let window = Window::new(1920, 1080, None).unwrap();
    let vulkan = Vulkan::new(&window).unwrap();
    let _ = window.render_loop(|| vulkan.draw());
}
