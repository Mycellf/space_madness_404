use crate::object::Object;
use macroquad::prelude::*;
use rapier2d::prelude::*;

pub mod app;
pub mod graphics;
pub mod keybinds;
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
        RigidBodyBuilder::dynamic().can_sleep(false).build(),
        ColliderBuilder::new(make_shape()).build(),
        load_texture("assets/ship.png").await.unwrap(),
    ));

    loop {
        app.check_fixed_tick();

        app.frame_tick();

        next_frame().await;
    }
}

// temp
fn make_shape() -> SharedShape {
    SharedShape::convex_decomposition(
        &vec![
            point![-8.0, 4.0],
            point![-4.0, 4.0],
            point![-2.0, 2.0],
            point![-2.0, 8.0],
            point![-1.0, 8.0],
            point![4.0, 3.0],
            point![6.0, 3.0],
            point![8.0, 1.0],
            point![8.0, -1.0],
            point![6.0, -3.0],
            point![4.0, -3.0],
            point![-1.0, -8.0],
            point![-2.0, -8.0],
            point![-2.0, -2.0],
            point![-4.0, -4.0],
            point![-8.0, -4.0],
        ],
        &vec![
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 4],
            [4, 5],
            [5, 6],
            [6, 7],
            [7, 8],
            [8, 9],
            [9, 10],
            [10, 11],
            [11, 12],
            [12, 13],
            [13, 14],
            [14, 15],
            [15, 0],
        ],
    )
}
