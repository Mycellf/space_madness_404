use crate::component::Component;
use crate::object::Object;
use crate::tilemap::{Tile, TileMap, TileType};
use macroquad::prelude::*;
use rapier2d::prelude::*;

pub mod app;
pub mod component;
pub mod graphics;
pub mod keybinds;
pub mod object;
pub mod physics_world;
pub mod tilemap;

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
        vec![
            Component::FaceMouse,
            Component::Motion {
                power: 100.0,
                brake: 0.975,
                emitter: Vec2::new(-8.0, 0.0),
            },
            Component::CameraFollow,
        ],
        Vec2::new(0.5, 0.5),
    ));

    app.objects.push(Object::new(
        &mut app.physics_world,
        RigidBodyBuilder::dynamic()
            .can_sleep(false)
            .translation(vector![40.0, 0.0])
            .build(),
        ColliderBuilder::new(make_shape()).build(),
        load_texture("assets/ship.png").await.unwrap(),
        Vec::new(),
        Vec2::new(0.5, 0.5),
    ));

    app.objects.push(Object::new(
        &mut app.physics_world,
        RigidBodyBuilder::fixed()
            .translation(vector![-64.0, -64.0])
            .build(),
        ColliderBuilder::new(make_shape()).build(),
        Texture2D::from_image(&Image::gen_image_color(
            (16 * Tile::SIZE_PIXELS) as u16,
            (16 * Tile::SIZE_PIXELS) as u16,
            BLANK,
        )),
        vec![Component::TileMap(TileMap::new(UVec2::new(16, 16)).await)],
        Vec2::new(0.0, 0.0),
    ));

    if let Component::TileMap(tile_map) = &mut app.objects[2].components[0] {
        tile_map.set(
            UVec2::new(1, 1),
            Tile {
                tile_type: TileType::Wall,
            },
        );
    }

    loop {
        app.check_fixed_tick();

        app.frame_tick();

        next_frame().await;
    }
}

// temp
fn make_shape() -> SharedShape {
    SharedShape::compound(vec![
        (
            Isometry::<Real>::identity(),
            SharedShape::convex_polyline(vec![
                point![-3.0, 2.0],
                point![-4.0, 3.0],
                point![-8.0, 3.0],
                point![-8.0, -3.0],
                point![-4.0, -3.0],
                point![-3.0, -2.0],
            ])
            .unwrap(),
        ),
        (
            Isometry::<Real>::identity(),
            SharedShape::convex_polyline(vec![
                point![-2.0, 4.0],
                point![-3.0, 3.0],
                point![-3.0, -3.0],
                point![-2.0, -4.0],
            ])
            .unwrap(),
        ),
        (
            Isometry::<Real>::identity(),
            SharedShape::convex_polyline(vec![
                point![4.0, 3.0],
                point![-1.0, 8.0],
                point![-2.0, 8.0],
                point![-2.0, -8.0],
                point![-1.0, -8.0],
                point![4.0, -3.0],
            ])
            .unwrap(),
        ),
        (
            Isometry::<Real>::identity(),
            SharedShape::convex_polyline(vec![
                point![8.0, 1.0],
                point![6.0, 3.0],
                point![4.0, 3.0],
                point![4.0, -3.0],
                point![6.0, -3.0],
                point![8.0, -1.0],
            ])
            .unwrap(),
        ),
    ])
}
