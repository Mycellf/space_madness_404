use macroquad::prelude::*;

pub mod app;

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Madness 404".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = app::App::new();

    let texture = load_texture("assets/ship.png").await.unwrap();

    loop {
        app.check_fixed_tick();

        app.frame_tick();

        draw_texture(&texture, 0.0, 0.0, WHITE);

        next_frame().await;
    }
}
