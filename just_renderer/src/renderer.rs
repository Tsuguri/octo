mod blit;
mod builtin_assets;
mod configuration;
pub mod gbuffer;
mod octo_postprocess;
mod pipeline_state;
mod renderer_single_frame;
mod runtime_configuration;

use std::ops::AddAssign;
use std::rc::Rc;
use std::sync::Arc;

pub use configuration::Configuration;
pub use octo_postprocess::{OctoModuleStatus, OctoPostprocessError};
use pipeline_state::PipelineState;
pub use renderer_single_frame::OverlayRenderContext;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use glam::{Quat, Vec3, Vec4};
use octo_runtime::OctoModule;

use crate::id_set::IdSet;
use crate::light::Light;
use crate::passes::generation::Generators;
use crate::shader_manager::ShaderManager;
use crate::{
    bind_group_layouts::BindGroupLayouts,
    camera::CameraData,
    material_library::{MaterialId, MaterialLibrary},
    model_data::ModelData,
    model_instance::{ModelId, ModelInstance},
    texture::TextureData,
    uniforms::{Model, ModelUniform, UtilData, UtilDataUniform},
};

use self::octo_postprocess::OctoPostprocessState;
use self::runtime_configuration::RuntimeConfiguration;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct InstanceId(u32);

impl AddAssign<u32> for InstanceId {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct LightId(u32);

impl AddAssign<u32> for LightId {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

pub type LightsCollection = IdSet<LightId, Light>;
pub type ObjectsCollection = IdSet<InstanceId, ModelInstance>;

pub struct Renderer {
    pub window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    configuration: Configuration,
    shaders: ShaderManager,

    rt_configuration: RuntimeConfiguration,
    octo_module: Option<OctoModule>,
    octo_postprocess: Option<OctoPostprocessState>,
    octo_module_status: OctoModuleStatus,
    octo_uniform_bytes: Vec<u8>,

    // all passes and textures that may depend on window size
    pipeline: Option<PipelineState>,
    generators: Generators,

    #[allow(dead_code)]
    builtin_textures: Vec<Rc<TextureData>>,
    game_textures: Vec<Rc<TextureData>>,

    materials: MaterialLibrary,
    lights: LightsCollection,

    builtin_models: Vec<ModelData>,
    game_models: Vec<ModelData>,
    objects: ObjectsCollection,

    pub camera_data: CameraData,

    bind_group_layouts: BindGroupLayouts,
    util_data_uniform: UtilDataUniform,

    #[cfg(target_arch = "wasm32")]
    url_origin: String,
}

impl Renderer {
    pub async fn initialize(event_loop: &ActiveEventLoop, configuration: Configuration) -> Self {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_inner_size(winit::dpi::LogicalSize::new(1024, 800))
                    .with_title("wolololo"),
            )
            .unwrap();

        let window = Arc::new(window);

        let size = window.inner_size();

        let (shaders, shaders_status) = ShaderManager::new("res/shaders/".to_owned());
        if let Result::Err(err) = shaders_status {
            log::error!("shaders were not compiled successfully!! {}", err);
        }

        let mut instance_descriptor = wgpu::InstanceDescriptor::new_with_display_handle(Box::new(
            event_loop.owned_display_handle(),
        ));
        #[cfg(target_arch = "wasm32")]
        {
            instance_descriptor.backends = wgpu::Backends::all();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            instance_descriptor.backends = wgpu::Backends::VULKAN;
        }

        let instance = wgpu::Instance::new(instance_descriptor);

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let optional_features = wgpu::Features::TEXTURE_BINDING_ARRAY | wgpu::Features::IMMEDIATES;
        let required_features = adapter.features() & optional_features;
        let mut required_limits = wgpu::Limits::default();
        if required_features.contains(wgpu::Features::TEXTURE_BINDING_ARRAY) {
            required_limits.max_binding_array_elements_per_shader_stage = 8;
        }
        if required_features.contains(wgpu::Features::IMMEDIATES) {
            required_limits.max_immediate_size = adapter.limits().max_immediate_size;
        }

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features,
                required_limits,
                memory_hints: Default::default(),
                label: None,
                ..Default::default()
            })
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let bind_group_layouts = BindGroupLayouts::initialize(&device);

        let util_data_uniform = UtilDataUniform::new(UtilData::new(), &device, &bind_group_layouts);

        let pos = Vec3::new(0.0, 2.0, -2.0);

        let camera_data = CameraData {
            position: pos,
            rotation: glam::Mat4::look_at_lh(pos, Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0))
                .to_scale_rotation_translation()
                .1,
            aspect_ratio: surface_config.width as f32 / surface_config.height as f32,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        };

        let materials = MaterialLibrary::new();

        let pipeline = PipelineState::create(
            &device,
            &surface_config,
            &bind_group_layouts,
            &shaders,
            &configuration,
        )
        .unwrap();

        let generators = Generators::initialize(&device, &bind_group_layouts);

        let (builtin_models, builtin_textures) = Self::load_builtin_data(
            &device,
            &queue,
            #[cfg(target_arch = "wasm32")]
            &url_origin,
        )
        .await;

        let objects = ObjectsCollection::empty();

        surface.configure(&device, &surface_config);

        let renderer = Renderer {
            window,
            surface,
            device,
            queue,
            surface_config,
            configuration,
            shaders,
            size,
            bind_group_layouts,
            util_data_uniform,
            objects,
            camera_data,
            pipeline: Some(pipeline),
            generators,
            builtin_models,
            game_models: Vec::new(),
            builtin_textures,
            game_textures: Vec::new(),
            materials,
            lights: LightsCollection::empty(),
            rt_configuration: Default::default(),
            octo_module: None,
            octo_postprocess: None,
            octo_module_status: OctoModuleStatus::Empty,
            octo_uniform_bytes: Vec::new(),
            #[cfg(target_arch = "wasm32")]
            url_origin,
        };

        return renderer;
    }

    pub fn runtime_configuration_mut(&mut self) -> &mut RuntimeConfiguration {
        &mut self.rt_configuration
    }

    pub fn set_octo_module(&mut self, module: OctoModule) -> Result<(), OctoPostprocessError> {
        log::info!("Loaded Octo module: {}", module.name);
        self.octo_uniform_bytes = vec![0; module.uniform_block_size];
        self.octo_module = Some(module);
        self.rebuild_octo_postprocess()
    }

    pub fn set_octo_uniform_bytes(&mut self, bytes: Vec<u8>) -> Result<(), OctoPostprocessError> {
        let Some(octo_postprocess) = &mut self.octo_postprocess else {
            return Err(OctoPostprocessError::new(
                "no ready octo module to receive uniforms",
            ));
        };

        octo_postprocess.set_uniform_bytes(bytes.clone())?;
        self.octo_uniform_bytes = bytes;
        Ok(())
    }

    pub fn octo_module(&self) -> Option<&OctoModule> {
        self.octo_module.as_ref()
    }

    pub fn octo_module_status(&self) -> OctoModuleStatus {
        self.octo_module_status.clone()
    }

    pub async fn load_game_model(&mut self, model_id: u32, path: &str) {
        assert_eq!(self.game_models.len(), model_id as usize);
        let obj_data = Self::load_file(path).await;
        self.game_models.push(ModelData::initialize_from_obj(
            &self.device,
            &obj_data,
            path,
        ));
    }

    pub async fn load_game_texture(&mut self, texture_id: u32, path: &str) {
        assert_eq!(self.game_textures.len(), texture_id as usize);
        let bytes = Self::load_file(path).await;
        let texture =
            Rc::new(TextureData::from_bytes(&self.device, &self.queue, &bytes, path).unwrap());
        self.game_textures.push(texture);
    }

    pub fn create_material(&mut self, texture_id: u32, color: Vec4) -> MaterialId {
        self.materials.add_material(
            &self.device,
            &self.bind_group_layouts,
            color,
            self.game_textures[texture_id as usize].clone(),
            // self.builtin_textures.last().unwrap().clone(),
        )
    }

    pub fn create_object_with_transform(
        &mut self,
        model_id: ModelId,
        material: MaterialId,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> InstanceId {
        let instance = ModelInstance {
            positon: position,
            rotation: rotation,
            scale: scale,
            model_id,
            model_uniform: ModelUniform::new(
                Model::default(),
                &self.device,
                &self.bind_group_layouts,
            ),
            material,
        };
        self.objects.insert(instance)
    }

    #[allow(dead_code)]
    pub fn create_object(&mut self, model_id: ModelId, material: MaterialId) -> InstanceId {
        self.create_object_with_transform(model_id, material, Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }

    pub fn object(&mut self, id: InstanceId) -> Option<&mut ModelInstance> {
        self.objects.get_mut(&id)
    }

    #[allow(dead_code)]
    pub fn remove_object(&mut self, id: InstanceId) {
        self.objects.remove(&id);
    }

    pub fn create_light(&mut self, light: Light) -> LightId {
        self.lights.insert(light)
    }

    pub fn light(&mut self, id: LightId) -> Option<&mut Light> {
        self.lights.get_mut(&id)
    }

    pub fn remove_light(&mut self, id: LightId) {
        self.lights.remove(&id);
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_file(file_name: &str, origin_url: &str) -> Vec<u8> {
        warn!("url: {}", origin_url);
        let base = reqwest::Url::parse(&format!("{}/", url_origin)).unwrap();
        let url = base.join(file_name).unwrap();
        let content = reqwest::get(url).await.unwrap().bytes().await.unwrap();
        content.to_vec()
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn load_file(file_name: &str) -> Vec<u8> {
        std::fs::read(file_name).unwrap()
    }

    pub fn pre_render(&mut self, time_delta: f32, total_time: f32) {
        self.util_data_uniform.modify(|data| {
            data.total_time = total_time;
            data.delta_time = time_delta;
            data.view = self.camera_data.view();
            data.projection = self.camera_data.projection();
            data.view_projection = self.camera_data.view_projection();
            data.eye_position = Vec4::from((self.camera_data.position, 0.0));
        });

        for instance in self.objects.values_mut() {
            instance.model_uniform.modify(|data| {
                let model_matrix = glam::Mat4::from_scale_rotation_translation(
                    instance.scale,
                    instance.rotation,
                    instance.positon,
                );
                data.model_matrix = model_matrix;
                data.normal_matrix = model_matrix.inverse().transpose();
            });
            instance.model_uniform.update(&self.queue);
        }
        self.util_data_uniform.update(&self.queue);

        if let Some(pipeline) = &mut self.pipeline {
            pipeline.postprocessing.update_buffers(
                &self.device,
                &self.queue,
                &self.bind_group_layouts,
                &self.lights,
            );
        }
    }

    pub fn post_render(&mut self) {}

    pub fn render(&mut self) -> Result<(), ()> {
        self.render_with_overlay(|_| Vec::new())
    }

    pub fn render_with_overlay(
        &mut self,
        overlay: impl FnOnce(OverlayRenderContext<'_>) -> Vec<wgpu::CommandBuffer>,
    ) -> Result<(), ()> {
        self.render_single_frame(overlay)
    }

    pub fn set_camera(&mut self, position: Vec3, rotation: Quat) {
        self.camera_data.position = position;
        self.camera_data.rotation = rotation;
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
    }

    pub fn surface_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    fn recreate_pipeline(&mut self) {
        log::info!("Recreating pipeline");
        self.pipeline = match PipelineState::create(
            &self.device,
            &self.surface_config,
            &self.bind_group_layouts,
            &self.shaders,
            &self.configuration,
        ) {
            Result::Ok(pipeline) => Some(pipeline),
            Result::Err(err) => {
                log::error!("Couldn't recreate pipeline: {}", err);
                None
            }
        };
        if self.octo_module.is_some() {
            let _ = self.rebuild_octo_postprocess();
        }
    }

    fn rebuild_octo_postprocess(&mut self) -> Result<(), OctoPostprocessError> {
        let Some(module) = self.octo_module.as_ref() else {
            self.octo_postprocess = None;
            self.octo_module_status = OctoModuleStatus::Empty;
            self.octo_uniform_bytes.clear();
            return Ok(());
        };

        match OctoPostprocessState::create(
            &self.device,
            self.surface_config.format,
            self.size,
            module,
        ) {
            Ok(mut octo_postprocess) => {
                if !self.octo_uniform_bytes.is_empty() {
                    octo_postprocess.set_uniform_bytes(self.octo_uniform_bytes.clone())?;
                }
                let name = octo_postprocess.name().to_owned();
                log::info!("Octo postprocessing pipeline is ready: {}", name);
                self.octo_postprocess = Some(octo_postprocess);
                self.octo_module_status = OctoModuleStatus::Ready { name };
                Ok(())
            }
            Err(error) => {
                log::error!("Couldn't prepare Octo postprocessing: {}", error);
                self.octo_postprocess = None;
                self.octo_module_status = OctoModuleStatus::Error {
                    name: module.name.clone(),
                    message: error.to_string(),
                };
                Err(error)
            }
        }
    }

    pub fn change_configuration(&mut self, new_configuration: Configuration) {
        log::trace!("Renderer configuration changed");
        self.configuration = new_configuration;
        self.recreate_pipeline();
    }

    pub fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 && self.size != *new_size {
            log::trace!(
                "changing surface size from {:?} to {:?}",
                self.size,
                *new_size
            );
            self.size = *new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
            self.camera_data.aspect_ratio =
                self.surface_config.width as f32 / self.surface_config.height as f32;
            self.recreate_pipeline();
        }
    }

    pub fn resurface(&mut self) {
        if self.size.width > 0 && self.size.height > 0 {
            self.surface.configure(&self.device, &self.surface_config);
            self.recreate_pipeline();
        }
    }
}
