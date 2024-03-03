use crate::physics_world::PhysicsWorld;
use macroquad::prelude::*;
use rapier2d::prelude::*;

pub struct Object {
    pub rigid_body: RigidBodyHandle,
    pub collider: ColliderHandle,
    pub texture: Texture2D,
}

impl Object {
    pub fn new(physics_world: &mut PhysicsWorld, shape: SharedShape, texture: Texture2D) -> Self {
        let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic).build();
        let collider = ColliderBuilder::new(shape).build();

        let (rigid_body, collider) = physics_world.add_rigidbody(rigid_body, collider);

        Self {
            rigid_body,
            collider,
            texture,
        }
    }
}
