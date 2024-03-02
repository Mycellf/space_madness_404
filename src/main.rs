use macroquad::prelude::*;

pub mod game_world;

const MAX_FRAME_TICKS: u32 = 5;
const FIXED_TICKS_PER_SEC: f32 = 60.0;

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

    let tri_pixel_material = load_material(
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
        zoom: Vec2::splat(1.0 / 128.0),
        ..Default::default()
    };

    let mut fixed_tick_time = 0.0;

    loop {
        fixed_tick_time += get_frame_time() * FIXED_TICKS_PER_SEC;

        for _ in 0..(fixed_tick_time as u32).min(MAX_FRAME_TICKS) {
            camera.target.x += 0.1;
        }

        fixed_tick_time %= 1.0;

        clear_background(BLACK);

        camera.zoom.x = camera.zoom.y / screen_width() * screen_height();
        set_camera(&camera);

        gl_use_material(&tri_pixel_material);

        draw_texture(&texture, 0.0, 0.0, WHITE);

        next_frame().await;
    }
}
