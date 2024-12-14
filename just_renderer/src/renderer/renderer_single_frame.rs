use crate::{model_data::ModelData, model_instance::ModelId};

use super::Renderer;

impl Renderer {
    pub(crate) fn render_single_frame(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
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

        Ok(())
    }
}
