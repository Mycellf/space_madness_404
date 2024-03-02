use macroquad::prelude::*;

pub fn make_tri_pixel_material() -> Material {
    load_material(
        ShaderSource::Glsl {
            vertex: include_str!("shaders/vertex.glsl"),
            fragment: include_str!("shaders/fragment.glsl"),
        },
        MaterialParams {
            pipeline_params: PipelineParams {
                color_blend: Some(miniquad::BlendState::new(
                    miniquad::Equation::Add,
                    miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                    miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .expect("Shader code should be valid")
}
