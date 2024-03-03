use crate::component::Component;
use crate::physics_world::PhysicsWorld;
use macroquad::prelude::*;
use rapier2d::prelude::*;

pub struct Object {
    pub rigid_body: RigidBodyHandle,
    pub collider: ColliderHandle,
    pub texture: Texture2D,
    pub size: Vec2,
    pub components: Vec<Component>,
}

impl Object {
    pub fn new(
        physics_world: &mut PhysicsWorld,
        rigid_body: RigidBody,
        collider: Collider,
        texture: Texture2D,
    ) -> Self {
        let (rigid_body, collider) = physics_world.add_rigidbody(rigid_body, collider);

        let size = texture.size() / 2.0;

        Self {
            rigid_body,
            collider,
            texture,
            size,
            components: Vec::new(),
        }
    }

    pub fn draw(&self, physics_world: &mut PhysicsWorld) {
        let rigid_body = &physics_world.rigid_body_set[self.rigid_body];

        let position: Vec2 = (*rigid_body.translation()).into();

        draw_texture_ex(
            &self.texture,
            position.x - self.size.x / 2.0,
            position.y - self.size.y / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                source: None,
                rotation: rigid_body.rotation().angle(),
                flip_x: false,
                flip_y: false,
                pivot: Some(position),
            },
        )
    }

    pub fn draw_info(&self, physics_world: &mut PhysicsWorld) {
        let rigid_body = &physics_world.rigid_body_set[self.rigid_body];
        let collider = &physics_world.collider_set[self.collider];

        let position: Vec2 = (*rigid_body.translation()).into();
        draw_marker_at(position, 0.8, 0.2, GREEN);

        let center_of_mass: Vec2 = (*rigid_body.center_of_mass()).into();
        draw_marker_at(center_of_mass, 1.0, 0.2, RED);
    }
}

fn draw_marker_at(position: Vec2, radius: f32, bold: f32, color: Color) {
    draw_line(
        position.x + radius,
        position.y,
        position.x - radius,
        position.y,
        bold,
        color,
    );

    draw_line(
        position.x,
        position.y + radius,
        position.x,
        position.y - radius,
        bold,
        color,
    );
}
