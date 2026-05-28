mod game;

use game::Scene;
use log::info;

use just_renderer::*;
use std::time::Instant;

use just_renderer::winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

fn initial_renderer_configuration() -> Configuration {
    Configuration { ssao: true }
}

struct GameApp {
    state: Option<ApplicationState>,
}

struct ApplicationState {
    renderer: Renderer,
    scene: Scene,
    timer: Instant,
    last_frame: Instant,
}

impl GameApp {
    fn new() -> Self {
        Self { state: None }
    }

    fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        self.state = Some(ApplicationState::initialize(event_loop));
    }
}

impl ApplicationState {
    fn initialize(event_loop: &ActiveEventLoop) -> Self {
        let mut renderer = pollster::block_on(Renderer::initialize(
            event_loop,
            initial_renderer_configuration(),
        ));
        let assets = pollster::block_on(game::resources::initialize_game_assets(&mut renderer));
        let scene = Scene::initialize_scene(&mut renderer, assets);
        let now = Instant::now();

        Self {
            renderer,
            scene,
            timer: now,
            last_frame: now,
        }
    }
}

impl ApplicationHandler for GameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.initialize(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        if state.renderer.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {}
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
                info!("pressed esc");
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: key_state,
                        physical_key: PhysicalKey::Code(key),
                        ..
                    },
                ..
            } => {
                state.scene.handle_keyboard_input(key, key_state);
            }

            WindowEvent::Resized(size) => {
                log::trace!("WindowEvent::Resized: {:?}", size);
                state.renderer.resize(&size);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        let dt = state.last_frame.elapsed().as_secs_f32();
        state.last_frame = Instant::now();
        let total_time = state.timer.elapsed().as_secs_f32();
        state.scene.update(&mut state.renderer, total_time);

        state.renderer.pre_render(dt, total_time);
        match state.renderer.render() {
            Ok(()) => {}
            Err(()) => event_loop.exit(),
        }
        state.renderer.post_render();
    }
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = GameApp::new();
    event_loop.run_app(&mut app).unwrap();
}
