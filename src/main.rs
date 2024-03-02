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

    let mut camera = Camera2D {
        zoom: Vec2::splat(1.0 / 64.0),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        camera.zoom.x = camera.zoom.y / screen_width() * screen_height();

        set_camera(&camera);

        draw_texture(&texture, 0.0, 0.0, WHITE);

        next_frame().await;
    }
}
