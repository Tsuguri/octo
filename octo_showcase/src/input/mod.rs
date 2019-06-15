use winit::{Event, EventsLoop, VirtualKeyCode, WindowEvent};
use crate::input::keyboard_state::{KeyboardState, KeyCode};

pub mod keyboard_state;

#[derive(Debug, Clone, Default)]
pub struct UserInput {
    pub end_requested: bool,
    pub new_frame_size: Option<(f64, f64)>,
    pub new_mouse_position: Option<(f64, f64)>,
}

impl UserInput {
    pub fn poll_events_loop(events_loop: &mut EventsLoop, keyboard_state: &mut KeyboardState) -> Self {
        let mut output = UserInput::default();
        keyboard_state.next_frame();
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => output.end_requested = true,
            Event::WindowEvent {
                event: WindowEvent::Resized(logical),
                ..
            } => {
                output.new_frame_size = Some((logical.width, logical.height));
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                output.new_mouse_position = Some((position.x, position.y));
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                //println!("pressed {:?}", input);


                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    output.end_requested = true;
                }
                let elem = match input.virtual_keycode {
                    Some(key) => key,
                    None => return,
                };
                keyboard_state.set_button(KeyCode::from_kc_enum(elem), input.state == winit::ElementState::Pressed);

            }
            _ => (),
        });
        output
    }
}
