use std::{
    cell::OnceCell,
    error::Error, 
    time::Instant,
};

use winit::{
    event::{
        Event as SystemEvent,
        WindowEvent as SystemWindowEvent,
    },
    event_loop::{
        EventLoop as SystemEventLoop,
        ControlFlow as SystemControlFlow,
    },
};

#[derive(Default)]
pub enum LoopStyle {
    Poll,
    #[default]
    Wait,
    WaitUntil(Instant),
}

//need to figure out the idiomatic patterns for how to appease borrow checker for
//struct fields
pub struct EventLoop {
    pub event_loop: OnceCell<SystemEventLoop<()>>,
}

impl EventLoop {
    pub fn new(loop_style: Option<LoopStyle>) -> Result<Self, Box<dyn Error>> {
        let event_loop = SystemEventLoop::new()?;
        let control_flow = match loop_style.unwrap_or_default() {
            LoopStyle::Poll => SystemControlFlow::Poll,
            LoopStyle::Wait => SystemControlFlow::Wait,
            LoopStyle::WaitUntil(instant) => SystemControlFlow::WaitUntil(instant),
        };
        event_loop.set_control_flow(control_flow);
        return Ok(Self{event_loop: event_loop.into()});
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let event_loop = self.event_loop.get().unwrap();
        let _ = event_loop.run(move |event, elwt| {
        match event {
            SystemEvent::WindowEvent {
                event: SystemWindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            _ => (),
        }});
        Ok(())
    }

}
