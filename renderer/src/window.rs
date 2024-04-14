/*
 * Inclinced to change the struct Window to System as
 * it seems more appropriate, keep calling window.window everywhere
 * which is really silly
*/
use std::{
    error::Error, 
    time::Instant,
    cell::RefCell,
};

use winit::{
    keyboard::{
        Key as SystemKey,
        NamedKey as SystemNamedKey,
    },
    event::{
        Event as SystemEvent,
        WindowEvent as SystemWindowEvent,
        KeyEvent as SystemKeyEvent,
        ElementState as SystemElementState,
    },
    event_loop::{
        EventLoop as SystemEventLoop,
        ControlFlow as SystemControlFlow,
    },
    window::{
        Window as SystemWindow,
        WindowBuilder as SystemWindowBuilder,
    },
    platform::run_on_demand::EventLoopExtRunOnDemand,
};

#[derive(Default)]
pub enum LoopStyle {
    #[default]
    Poll,
    Wait,
    WaitUntil(Instant),
}

pub struct Window {
    pub window: SystemWindow,
    pub event_loop: RefCell<SystemEventLoop<()>>,
    pub height: u32,
    pub width: u32
}

impl Window {
    pub fn new(window_width: u32, window_height: u32, loop_style: Option<LoopStyle>) 
            -> Result<Self, Box<dyn Error>> 
    {
        let event_loop = SystemEventLoop::new()?;
        let control_flow = match loop_style.unwrap_or_default() {
            LoopStyle::Poll => SystemControlFlow::Poll,
            LoopStyle::Wait => SystemControlFlow::Wait,
            LoopStyle::WaitUntil(instant) => SystemControlFlow::WaitUntil(instant),
        };
        event_loop.set_control_flow(control_flow);
        let window = SystemWindowBuilder::new()
            .with_title("Ash - Example")
            .with_inner_size(winit::dpi::LogicalSize::new(
                f64::from(window_width),
                f64::from(window_height),
            ))
            .build(&event_loop)
            .unwrap();
        Ok(Self { 
            window, 
            event_loop: RefCell::new(event_loop),
            height: window_height,
            width: window_width,
        })
    }

    //ripped directly from /home/dale/third_party/ash/ash-examples/src/bin/texture.rs
    pub fn render_loop<F: FnMut()>(&self, mut f: F) -> Result<(), impl Error> {
        self.event_loop.borrow_mut().run_on_demand(|event, elwp| {
            elwp.set_control_flow(SystemControlFlow::Poll);
            match event {
                SystemEvent::WindowEvent {
                    event:
                        SystemWindowEvent::CloseRequested
                        | SystemWindowEvent::KeyboardInput {
                            event:
                                SystemKeyEvent {
                                    state: SystemElementState::Pressed,
                                    logical_key: SystemKey::Named(SystemNamedKey::Escape),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    elwp.exit();
                }
                SystemEvent::AboutToWait => f(),
                _ => (),
            }
        })
    }
}
