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
    window::{Window, WindowId},
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
    ui_state: UiState,
    timer: Instant,
    last_frame: Instant,
}

struct UiOutput {
    textures_delta: egui::TexturesDelta,
    paint_jobs: Vec<egui::ClippedPrimitive>,
    screen_descriptor: egui_wgpu::ScreenDescriptor,
}

struct UiState {
    egui_context: egui::Context,
    egui_winit: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    pending_texture_frees: Vec<egui::TextureId>,
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
        let ui_state = UiState::new(&renderer);
        let now = Instant::now();

        Self {
            renderer,
            scene,
            ui_state,
            timer: now,
            last_frame: now,
        }
    }
}

impl UiState {
    fn new(renderer: &Renderer) -> Self {
        let egui_context = egui::Context::default();
        let egui_winit = egui_winit::State::new(
            egui_context.clone(),
            egui::ViewportId::ROOT,
            renderer.window.as_ref(),
            Some(renderer.window.scale_factor() as f32),
            renderer.window.theme(),
            None,
        );
        let egui_renderer = egui_wgpu::Renderer::new(
            renderer.device(),
            renderer.surface_format(),
            egui_wgpu::RendererOptions::default(),
        );

        Self {
            egui_context,
            egui_winit,
            egui_renderer,
            pending_texture_frees: Vec::new(),
        }
    }

    fn handle_window_event(
        &mut self,
        window: &Window,
        event: &WindowEvent,
    ) -> egui_winit::EventResponse {
        self.egui_winit.on_window_event(window, event)
    }

    fn build_output(
        &mut self,
        window: &Window,
        renderer: &mut Renderer,
        scene: &mut Scene,
    ) -> UiOutput {
        let raw_input = self.egui_winit.take_egui_input(window);
        let full_output = self.egui_context.run_ui(raw_input, |ui| {
            Self::construct_ui(ui, renderer, scene);
        });

        self.egui_winit
            .handle_platform_output(window, full_output.platform_output.clone());

        let paint_jobs = self
            .egui_context
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        let surface_size = renderer.surface_size();
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [surface_size.width, surface_size.height],
            pixels_per_point: full_output.pixels_per_point,
        };

        UiOutput {
            textures_delta: full_output.textures_delta,
            paint_jobs,
            screen_descriptor,
        }
    }

    fn construct_ui(ui: &mut egui::Ui, _renderer: &mut Renderer, _scene: &mut Scene) {
        egui::Panel::left("left_panel")
            .default_size(240.0)
            .show_inside(ui, |_ui| {});
    }

    fn render(
        &mut self,
        output: UiOutput,
        context: OverlayRenderContext<'_>,
    ) -> Vec<wgpu::CommandBuffer> {
        for id in self.pending_texture_frees.drain(..) {
            self.egui_renderer.free_texture(&id);
        }

        for (id, image_delta) in &output.textures_delta.set {
            self.egui_renderer
                .update_texture(context.device, context.queue, *id, image_delta);
        }

        let command_buffers = self.egui_renderer.update_buffers(
            context.device,
            context.queue,
            context.encoder,
            &output.paint_jobs,
            &output.screen_descriptor,
        );

        let render_pass = context
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui overlay render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: context.target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

        self.egui_renderer.render(
            &mut render_pass.forget_lifetime(),
            &output.paint_jobs,
            &output.screen_descriptor,
        );

        self.pending_texture_frees = output.textures_delta.free;

        command_buffers
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

        let egui_response = state
            .ui_state
            .handle_window_event(state.renderer.window.as_ref(), &event);

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
                if !egui_response.consumed {
                    state.scene.handle_keyboard_input(key, key_state);
                }
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
        let window = state.renderer.window.clone();
        let ui_output =
            state
                .ui_state
                .build_output(window.as_ref(), &mut state.renderer, &mut state.scene);
        match state
            .renderer
            .render_with_overlay(|context| state.ui_state.render(ui_output, context))
        {
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
