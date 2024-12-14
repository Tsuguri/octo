mod game;

use game::Scene;
use log::info;

use just_renderer::*;
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
};

fn initial_renderer_configuration() -> Configuration {
    Configuration { ssao: true }
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let mut renderer = Renderer::initialize(&event_loop, initial_renderer_configuration()).await;
    let assets = game::resources::initialize_game_assets(&mut renderer).await;
    let mut scene = Scene::initialize_scene(&mut renderer, assets);

    let timer = std::time::Instant::now();
    let mut last_frame = timer;
    event_loop.set_control_flow(ControlFlow::Poll); // I think

    event_loop
        .run(move |event, target| {
            match event {
                winit::event::Event::WindowEvent { ref event, .. } => {
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
                            target.exit();
                            info!("pressed esc");
                        }
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state,
                                    physical_key: PhysicalKey::Code(x),
                                    ..
                                },
                            ..
                        } => {
                            scene.handle_keyboard_input(*x, *state);
                        }

                        WindowEvent::Resized(size) => {
                            log::trace!("WindowEvent::Resized: {:?}", size);
                            renderer.resize(size);
                        }
                        // WindowEvent::ScaleFactorChanged {
                        //     scale_factor,
                        //     inner_size_writer,
                        // } => {
                        //     // let new_inner_size = inner_size_writer.new_inner_size;
                        //     // renderer.resize(*new_inner_size)
                        // }
                        _ => {}
                    }
                }
                winit::event::Event::AboutToWait => {
                    //renderer.window.request_redraw();
                    let dt = last_frame.elapsed().as_secs_f32();
                    last_frame = std::time::Instant::now();
                    scene.update(&mut renderer, timer.elapsed().as_secs_f32());

                    renderer.pre_render(dt, timer.elapsed().as_secs_f32());
                    match renderer.render() {
                        Ok(()) => {}
                        Err(()) => target.exit(),
                    }
                    renderer.post_render();
                }
                _ => {}
            };
        })
        .unwrap();
}
