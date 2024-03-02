use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Madness 404".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture = load_texture("assets/ship.png").await.unwrap();

    let fragment_shader = include_str!("shaders/fragment.glsl");
    let vertex_shader = include_str!("shaders/vertex.glsl");

    let pipeline_params = PipelineParams {
        color_blend: Some(miniquad::BlendState::new(
            miniquad::Equation::Add,
            miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
            miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
        )),
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: &vertex_shader,
            fragment: &fragment_shader,
        },
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();

    let mut camera = Camera2D {
        zoom: Vec2::splat(1.0 / 64.0),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        camera.zoom.x = camera.zoom.y / screen_width() * screen_height();
        set_camera(&camera);

        gl_use_material(&material);

        draw_texture(&texture, 0.0, 0.0, WHITE);

        next_frame().await;
    }
}
