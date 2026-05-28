use crate::{model_data::ModelData, model_instance::ModelId};

use super::Renderer;

pub struct OverlayRenderContext<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub target: &'a wgpu::TextureView,
    pub surface_size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    pub(crate) fn render_single_frame(
        &mut self,
        overlay: impl FnOnce(OverlayRenderContext<'_>) -> Vec<wgpu::CommandBuffer>,
    ) -> Result<(), ()> {
        let (output, should_resurface) = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => (output, false),
            wgpu::CurrentSurfaceTexture::Suboptimal(output) => (output, true),
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.resurface();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                log::error!("surface texture acquisition failed validation");
                return Err(());
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("empty encoder"),
            });

        self.generators.perlin.submit_jobs(&mut encoder);

        if self.rt_configuration.render_scene {
            if let Some(pipeline) = &mut self.pipeline {
                {
                    let mut solid_color_pass = pipeline.solid_color_pass.render(
                        &mut encoder,
                        &self.materials,
                        &pipeline.gbuffer,
                        &self.util_data_uniform,
                        true,
                    );
                    for object in self.objects.values() {
                        let model_data: &ModelData = match &object.model_id {
                            ModelId::BuiltIn(x) => &self.builtin_models[*x as usize],
                            ModelId::Game(x) => &self.game_models[*x as usize],
                        };
                        solid_color_pass.draw_single_object(model_data, object, object.material);
                    }
                }

                pipeline
                    .postprocessing
                    .render(&mut encoder, &view, &self.util_data_uniform);
            }
        }

        let overlay_command_buffers = overlay(OverlayRenderContext {
            device: &self.device,
            queue: &self.queue,
            encoder: &mut encoder,
            target: &view,
            surface_size: self.size,
        });

        self.queue.submit(
            overlay_command_buffers
                .into_iter()
                .chain([encoder.finish()]),
        );
        output.present();
        if should_resurface {
            self.resurface();
        }

        Ok(())
    }
}
