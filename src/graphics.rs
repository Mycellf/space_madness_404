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

pub fn draw_stars_around(center: Vec2) {
    let center = center.as_dvec2();

    for i in 500..700 {
        let i = (i * i) as f64;
        draw_star(
            DVec2::new(
                (i * 34501.0 - center.x) % 1000.0 - 500.0 + center.x,
                (i * 75683.0 - center.y) % 1000.0 - 500.0 + center.y,
            ),
            i * 4001.0,
        );
    }
}

fn draw_star(position: DVec2, rotation: f64) {
    draw_rectangle_ex(
        position.x as f32,
        position.y as f32,
        1.0,
        1.0,
        DrawRectangleParams {
            offset: Vec2::splat(0.5),
            rotation: rotation as f32,
            color: WHITE,
        },
    )
}
