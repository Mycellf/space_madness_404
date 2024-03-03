use crate::object::Object;
use macroquad::prelude::*;
use rapier2d::prelude::*;

pub mod app;
pub mod graphics;
pub mod object;
pub mod physics_world;

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

    app.objects.push(Object::new(
        &mut app.physics_world,
        ColliderBuilder::cuboid(8.0, 8.0).build(),
        load_texture("assets/ship.png").await.unwrap(),
    ));

    loop {
        app.check_fixed_tick();

        app.frame_tick();

        next_frame().await;
    }
}
