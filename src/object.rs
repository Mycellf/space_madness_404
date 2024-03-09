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
    pub offset: Vec2,
}

impl Object {
    pub fn new(
        physics_world: &mut PhysicsWorld,
        rigid_body: RigidBody,
        collider: Collider,
        texture: Texture2D,
        components: Vec<Component>,
        offset: Vec2,
    ) -> Self {
        let (rigid_body, collider) = physics_world.add_rigidbody(rigid_body, collider);

        let size = texture.size() / 2.0;

        Self {
            rigid_body,
            collider,
            texture,
            size,
            components,
            offset,
        }
    }

    pub fn draw(&self, physics_world: &mut PhysicsWorld) {
        let rigid_body = &physics_world.rigid_body_set[self.rigid_body];

        let position: Vec2 = (*rigid_body.translation()).into();

        draw_texture_ex(
            &self.texture,
            position.x - self.size.x * self.offset.x,
            position.y - self.size.y * self.offset.y,
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

    pub fn draw_debug(&self, physics_world: &mut PhysicsWorld) {
        let rigid_body = &physics_world.rigid_body_set[self.rigid_body];
        let collider = &physics_world.collider_set[self.collider];

        draw_compound_collider(collider, rigid_body.position());

        let position: Vec2 = (*rigid_body.translation()).into();
        draw_marker_at(position, 0.8, 0.2, GREEN);

        let center_of_mass: Vec2 = (*rigid_body.center_of_mass()).into();
        let velocity: Vec2 = (*rigid_body.linvel()).into();
        draw_velocity_marker_at(center_of_mass, velocity * 0.1, 1.0, 0.2, RED);
    }
}

fn draw_compound_collider(collider: &Collider, rigod_body_position: &Isometry<Real>) -> Option<()> {
    for (collider_position, shape) in collider.shape().as_compound()?.shapes() {
        if let Some(shape) = shape.as_convex_polygon() {
            let points: Vec<_> = shape
                .points()
                .into_iter()
                .map(|point| {
                    rigod_body_position.transform_point(&collider_position.transform_point(point))
                })
                .collect();

            for point in &points {
                draw_marker_at((*point).into(), 0.5, 0.1, MAGENTA)
            }

            for i in 0..points.len() {
                let a = points[i];
                let b = points[(i + 1) % points.len()];

                draw_line(a.x, a.y, b.x, b.y, 0.1, MAGENTA);
            }
        }
    }

    Some(())
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

fn draw_velocity_marker_at(position: Vec2, velocity: Vec2, radius: f32, bold: f32, color: Color) {
    draw_line(
        position.x + velocity.x.min(0.0) - radius,
        position.y,
        position.x + velocity.x.max(0.0) + radius,
        position.y,
        bold,
        color,
    );

    draw_line(
        position.x,
        position.y + velocity.y.min(0.0) - radius,
        position.x,
        position.y + velocity.y.max(0.0) + radius,
        bold,
        color,
    );
}
