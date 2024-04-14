use renderer::window::Window;
use renderer::vulkan::Vulkan;

fn main() {
    //TODO these shouldn't be called (Window|Vulkan)::new()
    let window = Window::new(1920, 1080, None).unwrap();
    let vulkan = Vulkan::new(&window).unwrap();
    let draw = vulkan.get_draw_fn();
    let _ = window.render_loop(draw);
}
