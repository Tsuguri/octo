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
    renderer: Option<Renderer>,
    scene: Option<Scene>,
    timer: Instant,
    last_frame: Instant,
}

impl GameApp {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            renderer: None,
            scene: None,
            timer: now,
            last_frame: now,
        }
    }

    fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            return;
        }

        let mut renderer = pollster::block_on(Renderer::initialize(
            event_loop,
            initial_renderer_configuration(),
        ));
        let assets = pollster::block_on(game::resources::initialize_game_assets(&mut renderer));
        let scene = Scene::initialize_scene(&mut renderer, assets);

        self.renderer = Some(renderer);
        self.scene = Some(scene);
        self.timer = Instant::now();
        self.last_frame = self.timer;
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
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };
        if renderer.window.id() != window_id {
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
                        state,
                        physical_key: PhysicalKey::Code(key),
                        ..
                    },
                ..
            } => {
                if let Some(scene) = self.scene.as_mut() {
                    scene.handle_keyboard_input(key, state);
                }
            }

            WindowEvent::Resized(size) => {
                log::trace!("WindowEvent::Resized: {:?}", size);
                renderer.resize(&size);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let (Some(renderer), Some(scene)) = (self.renderer.as_mut(), self.scene.as_mut()) else {
            return;
        };

        let dt = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
        let total_time = self.timer.elapsed().as_secs_f32();
        scene.update(renderer, total_time);

        renderer.pre_render(dt, total_time);
        match renderer.render() {
            Ok(()) => {}
            Err(()) => event_loop.exit(),
        }
        renderer.post_render();
    }
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = GameApp::new();
    event_loop.run_app(&mut app).unwrap();
}
