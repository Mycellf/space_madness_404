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

    const TILE_MAP_SIZE: u32 = 16;

    app.objects.push(Object::new(
        &mut app.physics_world,
        RigidBodyBuilder::fixed()
            .translation(vector![-64.0, -64.0])
            .build(),
        ColliderBuilder::new(make_shape()).build(),
        Texture2D::from_image(&Image::gen_image_color(
            (TILE_MAP_SIZE * Tile::SIZE_PIXELS) as u16,
            (TILE_MAP_SIZE * Tile::SIZE_PIXELS) as u16,
            BLANK,
        )),
        vec![Component::TileMap(
            TileMap::new(uvec2(TILE_MAP_SIZE, TILE_MAP_SIZE)).await,
        )],
        vec2(0.0, 0.0),
    ));

    if let Component::TileMap(tile_map) = &mut app.objects[0].components[0] {
        for x in 0..=4 {
            for y in 0..=4 {
                tile_map.set(
                    uvec2(x, y),
                    Tile {
                        tile_type: TileType::Wall,
                    },
                );
            }
        }
    }

    app.objects.push(Object::new(
        &mut app.physics_world,
        RigidBodyBuilder::dynamic()
            .ccd_enabled(true)
            .can_sleep(false)
            .build(),
        ColliderBuilder::new(make_shape()).build(),
        load_texture("assets/ship_active.png").await.unwrap(),
        vec![
            Component::FaceMouse,
            Component::Motion {
                power: 100.0,
                brake: 0.975,
                emitter: vec2(-8.0, 0.0),
            },
            Component::CameraFollow,
        ],
        vec2(0.5, 0.5),
    ));

    app.objects.push(Object::new(
        &mut app.physics_world,
        RigidBodyBuilder::dynamic()
            .ccd_enabled(true)
            .can_sleep(false)
            .translation(vector![40.0, 0.0])
            .build(),
        ColliderBuilder::new(make_shape()).build(),
        load_texture("assets/ship_inactive.png").await.unwrap(),
        Vec::new(),
        vec2(0.5, 0.5),
    ));

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
