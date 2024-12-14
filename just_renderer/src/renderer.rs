mod builtin_assets;
mod configuration;
pub mod gbuffer;
mod pipeline_state;
mod renderer_single_frame;
mod runtime_configuration;

use std::ops::AddAssign;
use std::sync::Arc;
use std::{collections::HashMap, rc::Rc};

pub use configuration::Configuration;
use pipeline_state::PipelineState;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use glam::{Quat, Vec3, Vec4};

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
    pub async fn initialize(event_loop: &EventLoop<()>, configuration: Configuration) -> Self {
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(1024, 800))
            .with_title("wolololo")
            .build(&event_loop)
            .unwrap();

        let window = Arc::new(window);

        let size = window.inner_size();

        let (shaders, shaders_status) = ShaderManager::new("res/shaders/".to_owned());
        if let Result::Err(err) = shaders_status {
            log::error!("shaders were not compiled successfully!! {}", err);
        }

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::all(),
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(window.clone()) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits {
                        max_push_constant_size: 64,
                        ..Default::default()
                    },
                    memory_hints: Default::default(),
                    label: None,
                },
                None,
            )
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

        let mut generators = Generators::initialize(&device, &bind_group_layouts);

        let (builtin_models, mut builtin_textures) = Self::load_builtin_data(
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
            #[cfg(target_arch = "wasm32")]
            url_origin,
        };

        return renderer;
    }

    pub fn runtime_configuration_mut(&mut self) -> &mut RuntimeConfiguration {
        &mut self.rt_configuration
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
        match self.render_single_frame() {
            Err(wgpu::SurfaceError::Lost) => {
                self.resurface();
                Ok(())
            }
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => Err(()),
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => {
                eprintln!("{:?}", e);
                Err(())
            }
            Ok(()) => Ok(()),
        }
    }

    pub fn set_camera(&mut self, position: Vec3, rotation: Quat) {
        self.camera_data.position = position;
        self.camera_data.rotation = rotation;
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
        let size = self.size;
        self.resize(&size);
    }
}
