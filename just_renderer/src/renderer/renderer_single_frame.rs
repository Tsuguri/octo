use crate::{model_data::ModelData, model_instance::ModelId};

use super::Renderer;

impl Renderer {
    pub(crate) fn render_single_frame(&mut self) -> Result<(), ()> {
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
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        if should_resurface {
            self.resurface();
        }

        Ok(())
    }
}
