use crate::keybinds::*;
use crate::object::Object;
use crate::physics_world::PhysicsWorld;
use macroquad::prelude::*;
use rapier2d::prelude::*;

pub struct App {
    pub paused: bool,
    pub debug: bool,
    pub fixed_tick_time: f32,
    pub camera: Camera2D,
    pub material: Material,
    pub keybinds: Keybinds,
    pub physics_world: PhysicsWorld,
    pub objects: Vec<Object>,
}

impl App {
    pub const FIXED_TICKS_PER_SEC: f32 = 60.0;
    pub const FIXED_DELTA_TIME: f32 = 1.0 / Self::FIXED_TICKS_PER_SEC;
    pub const MAX_TICKS_PER_FRAME: u32 = 5;

    pub fn new() -> Self {
        Self {
            paused: false,
            debug: false,
            fixed_tick_time: 0.0,
            camera: Camera2D {
                zoom: Vec2::splat(1.0 / 64.0),
                ..Default::default()
            },
            material: crate::graphics::make_tri_pixel_material(),
            keybinds: Keybinds::default(),
            physics_world: PhysicsWorld::new(),
            objects: Vec::new(),
        }
    }

    pub fn frame_tick(&mut self) {
        self.keybinds.update();

        if self.keybinds.get(KeyAction::Pause).is_just_pressed() {
            self.paused ^= true;
        }

        if self.keybinds.get(KeyAction::Debug).is_just_pressed() {
            self.debug ^= true;
        }

        unsafe {
            let app_ptr = self as *mut App;
            for object in &mut self.objects {
                let object_ptr = object as *mut Object;
                for component in &object.components {
                    component.frame_update(object_ptr.as_mut().unwrap(), app_ptr.as_mut().unwrap());
                }
            }
        }

        self.update_camera();

        clear_background(BLACK);

        gl_use_material(&self.material);

        for object in &self.objects {
            object.draw(&mut self.physics_world);
        }

        gl_use_default_material();

        if self.debug {
            for object in &self.objects {
                object.draw_info(&mut self.physics_world);
            }
        }
    }

    pub fn check_fixed_tick(&mut self) {
        if self.paused {
            return;
        }

        self.fixed_tick_time += get_frame_time() * Self::FIXED_TICKS_PER_SEC;

        for _ in 0..(self.fixed_tick_time as u32).min(Self::MAX_TICKS_PER_FRAME) {
            self.fixed_tick();
        }

        self.fixed_tick_time %= 1.0;
    }

    fn fixed_tick(&mut self) {
        unsafe {
            let app_ptr = self as *mut App;
            for object in &mut self.objects {
                let object_ptr = object as *mut Object;
                for component in &object.components {
                    component.fixed_update(object_ptr.as_mut().unwrap(), app_ptr.as_mut().unwrap());
                }
            }
        }

        self.physics_world.step();
    }

    fn update_camera(&mut self) {
        self.camera.zoom.x = self.camera.zoom.y / screen_width() * screen_height();
        set_camera(&self.camera);
    }

    pub fn get_rigid_body(&self, object: &Object) -> &RigidBody {
        &self.physics_world.rigid_body_set[object.rigid_body]
    }

    pub fn get_rigid_body_mut(&mut self, object: &Object) -> &mut RigidBody {
        &mut self.physics_world.rigid_body_set[object.rigid_body]
    }

    pub fn get_collider(&self, object: &Object) -> &Collider {
        &self.physics_world.collider_set[object.collider]
    }

    pub fn get_collider_mut(&mut self, object: &Object) -> &mut Collider {
        &mut self.physics_world.collider_set[object.collider]
    }
}
