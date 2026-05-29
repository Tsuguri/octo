use game::Scene;
use just_renderer::winit::{event::WindowEvent, window::Window};
use just_renderer::{
    wgpu, OctoAutoUniformValue, OctoModule, OctoModuleStatus, OverlayRenderContext, Renderer,
    ValueType,
};
use std::sync::mpsc::{Receiver, TryRecvError};

use crate::game;

type LoadResult = Result<Option<OctoModule>, String>;

struct UiUniformState {
    values: Vec<UiUniform>,
}

impl UiUniformState {
    fn from_module(module: &OctoModule, renderer: &Renderer) -> Self {
        Self::from_module_with_auto_classifier(module, |name, value_type| {
            renderer.is_octo_uniform_auto_filled(name, value_type)
        })
    }

    fn from_module_with_auto_classifier(
        module: &OctoModule,
        is_auto_filled: impl Fn(&str, ValueType) -> bool,
    ) -> Self {
        Self {
            values: module
                .uniform_block
                .iter()
                .map(|(name, value_type)| UiUniform {
                    name: name.clone(),
                    value: UniformValue::default_for(*value_type),
                    auto_filled: is_auto_filled(name, *value_type),
                })
                .collect(),
        }
    }

    fn sync_auto_values(&mut self, renderer: &Renderer) {
        for uniform in &mut self.values {
            uniform.sync_auto_value(renderer);
        }
    }

    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for uniform in &self.values {
            uniform.value.write_bytes(&mut bytes);
        }
        bytes
    }
}

struct UiUniform {
    name: String,
    value: UniformValue,
    auto_filled: bool,
}

impl UiUniform {
    fn draw(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            ui.label(&self.name);
            if self.auto_filled {
                ui.add_enabled_ui(false, |ui| {
                    self.value.draw(ui);
                });
            } else {
                changed = self.value.draw(ui);
            }
        });
        changed
    }

    fn sync_auto_value(&mut self, renderer: &Renderer) {
        if !self.auto_filled {
            return;
        }

        if let Some(value) = renderer.octo_auto_uniform_value(&self.name, self.value.value_type()) {
            self.value.set_auto_value(value);
        }
    }
}

enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat3([f32; 9]),
    Mat4([f32; 16]),
    Int(i32),
    Bool(bool),
}

impl UniformValue {
    fn default_for(value_type: ValueType) -> Self {
        match value_type {
            ValueType::Float => Self::Float(0.0),
            ValueType::Vec2 => Self::Vec2([0.0; 2]),
            ValueType::Vec3 => Self::Vec3([0.0; 3]),
            ValueType::Vec4 => Self::Vec4([0.0; 4]),
            ValueType::Mat3 => Self::Mat3([
                1.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, //
                0.0, 0.0, 1.0,
            ]),
            ValueType::Mat4 => Self::Mat4([
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0,
            ]),
            ValueType::Int => Self::Int(0),
            ValueType::Bool => Self::Bool(false),
        }
    }

    fn draw(&mut self, ui: &mut egui::Ui) -> bool {
        match self {
            Self::Float(value) => ui.add(egui::DragValue::new(value).speed(0.01)).changed(),
            Self::Vec2(values) => draw_f32_slice(ui, values),
            Self::Vec3(values) => draw_f32_slice(ui, values),
            Self::Vec4(values) => draw_f32_slice(ui, values),
            Self::Mat3(values) => draw_f32_slice(ui, values),
            Self::Mat4(values) => draw_f32_slice(ui, values),
            Self::Int(value) => ui.add(egui::DragValue::new(value)).changed(),
            Self::Bool(value) => ui.checkbox(value, "").changed(),
        }
    }

    fn value_type(&self) -> ValueType {
        match self {
            Self::Float(_) => ValueType::Float,
            Self::Vec2(_) => ValueType::Vec2,
            Self::Vec3(_) => ValueType::Vec3,
            Self::Vec4(_) => ValueType::Vec4,
            Self::Mat3(_) => ValueType::Mat3,
            Self::Mat4(_) => ValueType::Mat4,
            Self::Int(_) => ValueType::Int,
            Self::Bool(_) => ValueType::Bool,
        }
    }

    fn set_auto_value(&mut self, value: OctoAutoUniformValue) {
        match (self, value) {
            (Self::Vec2(target), OctoAutoUniformValue::Vec2(value)) => *target = value,
            (Self::Vec3(target), OctoAutoUniformValue::Vec3(value)) => *target = value,
            _ => {}
        }
    }

    fn write_bytes(&self, bytes: &mut Vec<u8>) {
        match self {
            Self::Float(value) => write_padded(bytes, &value.to_ne_bytes(), 16),
            Self::Vec2(values) => write_f32s_padded(bytes, values, 16),
            Self::Vec3(values) => write_f32s_padded(bytes, values, 16),
            Self::Vec4(values) => write_f32s_padded(bytes, values, 16),
            Self::Mat3(values) => write_f32s_padded(bytes, values, 36),
            Self::Mat4(values) => write_f32s_padded(bytes, values, 64),
            Self::Int(value) => write_padded(bytes, &value.to_ne_bytes(), 16),
            Self::Bool(value) => {
                let value = u32::from(*value);
                write_padded(bytes, &value.to_ne_bytes(), 16);
            }
        }
    }
}

fn draw_f32_slice<const N: usize>(ui: &mut egui::Ui, values: &mut [f32; N]) -> bool {
    let mut changed = false;
    for value in values {
        changed |= ui
            .add(egui::DragValue::new(value).speed(0.01).max_decimals(3))
            .changed();
    }
    changed
}

fn write_f32s_padded<const N: usize>(bytes: &mut Vec<u8>, values: &[f32; N], padded_size: usize) {
    let start = bytes.len();
    for value in values {
        bytes.extend_from_slice(&value.to_ne_bytes());
    }
    bytes.resize(start + padded_size, 0);
}

fn write_padded(bytes: &mut Vec<u8>, value: &[u8], padded_size: usize) {
    let start = bytes.len();
    bytes.extend_from_slice(value);
    bytes.resize(start + padded_size, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_uniforms_with_octo_padding() {
        let mut module = OctoModule::new();
        module.uniform_block = vec![
            ("exposure".to_owned(), ValueType::Float),
            ("enabled".to_owned(), ValueType::Bool),
            ("transform".to_owned(), ValueType::Mat3),
        ];

        let uniforms = UiUniformState::from_module_with_auto_classifier(&module, |_, _| false);
        let bytes = uniforms.bytes();

        assert_eq!(bytes.len(), 68);
        assert_eq!(&bytes[0..4], &0.0f32.to_ne_bytes());
        assert_eq!(&bytes[16..20], &0u32.to_ne_bytes());
        assert_eq!(&bytes[32..36], &1.0f32.to_ne_bytes());
        assert_eq!(&bytes[48..52], &1.0f32.to_ne_bytes());
        assert_eq!(&bytes[64..68], &1.0f32.to_ne_bytes());
    }

    #[test]
    fn auto_uniforms_are_marked_by_exact_name_and_type() {
        let mut module = OctoModule::new();
        module.uniform_block = vec![
            ("camera_pos".to_owned(), ValueType::Vec3),
            ("lightDir".to_owned(), ValueType::Float),
            ("view_size".to_owned(), ValueType::Vec2),
            ("exposure".to_owned(), ValueType::Float),
        ];

        let uniforms =
            UiUniformState::from_module_with_auto_classifier(&module, |name, value_type| {
                matches!(
                    (name, value_type),
                    ("camera_pos", ValueType::Vec3) | ("view_size", ValueType::Vec2)
                )
            });

        assert!(uniforms.values[0].auto_filled);
        assert!(!uniforms.values[1].auto_filled);
        assert!(uniforms.values[2].auto_filled);
        assert!(!uniforms.values[3].auto_filled);
    }

    #[test]
    fn auto_values_update_matching_uniform_value_types() {
        let mut vec2 = UniformValue::Vec2([0.0, 0.0]);
        vec2.set_auto_value(OctoAutoUniformValue::Vec2([1280.0, 720.0]));
        assert!(matches!(vec2, UniformValue::Vec2([1280.0, 720.0])));

        let mut vec3 = UniformValue::Vec3([0.0, 0.0, 0.0]);
        vec3.set_auto_value(OctoAutoUniformValue::Vec3([1.0, 2.0, 3.0]));
        assert!(matches!(vec3, UniformValue::Vec3([1.0, 2.0, 3.0])));

        vec3.set_auto_value(OctoAutoUniformValue::Vec2([9.0, 9.0]));
        assert!(matches!(vec3, UniformValue::Vec3([1.0, 2.0, 3.0])));
    }
}

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
    uniforms: Option<UiUniformState>,
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
            uniforms: None,
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

                ui.separator();
                self.draw_octo_status(ui, renderer);
                self.draw_uniforms(ui, renderer);
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
                self.uniforms = Some(UiUniformState::from_module(&module, renderer));
                match renderer.set_octo_module(module) {
                    Ok(()) => {
                        if let Some(uniforms) = &self.uniforms {
                            if let Err(error) = renderer.set_octo_uniform_bytes(uniforms.bytes()) {
                                log::error!("Couldn't upload Octo uniforms: {}", error);
                            }
                        }
                        self.load_error = None;
                    }
                    Err(error) => {
                        log::error!("Couldn't prepare Octo module: {}", error);
                    }
                }
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

    fn draw_octo_status(&self, ui: &mut egui::Ui, renderer: &Renderer) {
        match renderer.octo_module_status() {
            OctoModuleStatus::Empty => {
                ui.label("No Octo module loaded");
            }
            OctoModuleStatus::Ready { name } => {
                ui.label(format!("Octo module: {name}"));
            }
            OctoModuleStatus::Error { name, message } => {
                ui.label(format!("Octo module: {name}"));
                ui.colored_label(egui::Color32::LIGHT_RED, message);
            }
        }
    }

    fn draw_uniforms(&mut self, ui: &mut egui::Ui, renderer: &mut Renderer) {
        let Some(uniforms) = &mut self.uniforms else {
            return;
        };

        uniforms.sync_auto_values(renderer);

        if uniforms.values.is_empty() {
            return;
        }

        ui.separator();
        ui.label("Uniforms");

        let mut changed = false;
        for uniform in &mut uniforms.values {
            changed |= uniform.draw(ui);
        }

        if changed {
            if let Err(error) = renderer.set_octo_uniform_bytes(uniforms.bytes()) {
                log::error!("Couldn't upload Octo uniforms: {}", error);
            }
        }
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
