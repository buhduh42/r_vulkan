use std::error::Error;

use winit::{
    window::Window as SystemWindow,
    window::WindowBuilder as SystemWindowBuilder,
};

use super::event_loop::EventLoop;

pub struct Window {
    window: SystemWindow,
}

impl Window {
    pub fn new(window_width: u32, window_height: u32, event_loop: &EventLoop) 
            -> Result<Self, Box<dyn Error>> 
    {
        let window = SystemWindowBuilder::new()
            .with_title("Ash - Example")
            .with_inner_size(winit::dpi::LogicalSize::new(
                f64::from(window_width),
                f64::from(window_height),
            ))
            .build(&event_loop.event_loop.get().unwrap())
            .unwrap();
        return Ok(Self { window: window });
    }
}
