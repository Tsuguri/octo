//extern crate log;
#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

use winit::{EventsLoop, Window, WindowBuilder, Event, WindowEvent, CreationError};

#[derive(Debug)]
struct WinitState {
    pub events_loop: EventsLoop,
    pub window: Window,
}

impl WinitState {
    pub fn new<T: Into<String>>(title: T, size: winit::dpi::LogicalSize) -> Result<Self, CreationError> {

        let mut events_loop = EventsLoop::new();
        let output = WindowBuilder::new().with_title(title).with_dimensions(size).build(&events_loop);
        output.map(|window| Self {events_loop, window,})

    }

}

pub const window_name: &str = "Hello octo";

impl Default for WinitState {
    fn default() -> Self {
        Self::new(window_name, winit::dpi::LogicalSize {width: 800.0, height: 600.0,}).expect("Could not create a window!")
    }
}

fn main() {
    simple_logger::init().unwrap();

    info!("whoop whoop");

    let mut state = WinitState::default();


    let mut running = true;

    while running {
        state.events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..} => running = false,
                _ => (),
        });
    }
}
