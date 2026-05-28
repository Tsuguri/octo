use game::Scene;
use just_renderer::winit::{event::WindowEvent, window::Window};
use just_renderer::{wgpu, OctoModule, OverlayRenderContext, Renderer};
use std::sync::mpsc::{Receiver, TryRecvError};

use crate::game;

type LoadResult = Result<Option<OctoModule>, String>;

pub(crate) struct UiOutput {
    textures_delta: egui::TexturesDelta,
    paint_jobs: Vec<egui::ClippedPrimitive>,
    screen_descriptor: egui_wgpu::ScreenDescriptor,
}

pub(crate) struct UiState {
    egui_context: egui::Context,
    egui_winit: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    pending_texture_frees: Vec<egui::TextureId>,
    load_error: Option<String>,
    pending_load: Option<Receiver<LoadResult>>,
}

impl UiState {
    pub(crate) fn new(renderer: &Renderer) -> Self {
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
            load_error: None,
            pending_load: None,
        }
    }

    pub(crate) fn handle_window_event(
        &mut self,
        window: &Window,
        event: &WindowEvent,
    ) -> egui_winit::EventResponse {
        self.egui_winit.on_window_event(window, event)
    }

    pub(crate) fn build_output(
        &mut self,
        window: &Window,
        renderer: &mut Renderer,
        scene: &mut Scene,
    ) -> UiOutput {
        let raw_input = self.egui_winit.take_egui_input(window);
        let egui_context = self.egui_context.clone();
        let full_output = egui_context.run_ui(raw_input, |ui| {
            self.construct_ui(ui, renderer, scene);
        });

        self.egui_winit
            .handle_platform_output(window, full_output.platform_output.clone());

        let paint_jobs = egui_context.tessellate(full_output.shapes, full_output.pixels_per_point);
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

    fn construct_ui(&mut self, ui: &mut egui::Ui, renderer: &mut Renderer, _scene: &mut Scene) {
        self.poll_octo_bin_load(renderer);

        egui::Panel::left("left_panel")
            .default_size(240.0)
            .show_inside(ui, |ui| {
                let is_loading = self.pending_load.is_some();
                let button_text = if is_loading {
                    "Loading .octo_bin..."
                } else {
                    "Load .octo_bin"
                };

                if ui
                    .add_enabled(!is_loading, egui::Button::new(button_text))
                    .clicked()
                {
                    if let Err(error) = self.start_octo_bin_load() {
                        log::error!("Couldn't load .octo_bin: {}", error);
                        self.load_error = Some(error);
                    }
                }
            });

        if let Some(error) = self.load_error.clone() {
            let mut is_open = true;
            egui::Window::new("Couldn't load .octo_bin")
                .collapsible(false)
                .resizable(false)
                .open(&mut is_open)
                .show(ui.ctx(), |ui| {
                    ui.label(error);
                    if ui.button("Close").clicked() {
                        self.load_error = None;
                    }
                });

            if !is_open {
                self.load_error = None;
            }
        }
    }

    fn start_octo_bin_load(&mut self) -> Result<(), String> {
        if self.pending_load.is_some() {
            return Ok(());
        }

        let task = rfd::AsyncFileDialog::new()
            .add_filter("Octo module", &["octo_bin"])
            .pick_file();
        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::Builder::new()
            .name("octo_bin_file_dialog".to_owned())
            .spawn(move || {
                let result = pollster::block_on(async move {
                    let Some(file) = task.await else {
                        return Ok(None);
                    };

                    let path = file.path().to_path_buf();
                    let module = UiState::read_octo_module(&path).map_err(|error| {
                        let file_name = path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("selected file");
                        format!("{file_name}: {error}")
                    })?;

                    Ok(Some(module))
                });

                let _ = sender.send(result);
            })
            .map_err(|error| format!("couldn't start file dialog task: {error}"))?;

        self.pending_load = Some(receiver);
        Ok(())
    }

    fn poll_octo_bin_load(&mut self, renderer: &mut Renderer) {
        let Some(receiver) = self.pending_load.as_ref() else {
            return;
        };

        match receiver.try_recv() {
            Ok(Ok(Some(module))) => {
                renderer.set_octo_module(module);
                self.load_error = None;
                self.pending_load = None;
            }
            Ok(Ok(None)) => {
                self.pending_load = None;
            }
            Ok(Err(error)) => {
                log::error!("Couldn't load .octo_bin: {}", error);
                self.load_error = Some(error);
                self.pending_load = None;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                let error = "file dialog task ended unexpectedly".to_owned();
                log::error!("Couldn't load .octo_bin: {}", error);
                self.load_error = Some(error);
                self.pending_load = None;
            }
        }
    }

    fn read_octo_module(path: &std::path::Path) -> Result<OctoModule, String> {
        let data = std::fs::read_to_string(path)
            .map_err(|error| format!("couldn't read file: {error}"))?;
        serde_json::from_str(&data).map_err(|error| format!("couldn't deserialize module: {error}"))
    }

    pub(crate) fn render(
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
