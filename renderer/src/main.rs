pub mod system;

fn main() {
    let event_loop = system::event_loop::EventLoop::new(None)
        .unwrap();
    system::init(1920, 1080, &event_loop);
    event_loop.run();
}

/*
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    //let mut event_loop = EventLoop::new().unwrap();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let _window = Window::new(&event_loop).unwrap();

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            _ => (),
        }
    });
}
*/
