use crate::keybinds::*;
use crate::object::Object;
use crate::physics_world::PhysicsWorld;
use macroquad::prelude::*;

pub struct App {
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

        self.update_camera();

        clear_background(BLACK);

        gl_use_material(&self.material);

        for object in &self.objects {
            object.draw(&mut self.physics_world);
        }
    }

    pub fn check_fixed_tick(&mut self) {
        self.fixed_tick_time += get_frame_time() * Self::FIXED_TICKS_PER_SEC;

        for _ in 0..(self.fixed_tick_time as u32).min(Self::MAX_TICKS_PER_FRAME) {
            self.fixed_tick();
        }

        self.fixed_tick_time %= 1.0;
    }

    fn fixed_tick(&mut self) {
        self.physics_world.step();
    }

    fn update_camera(&mut self) {
        self.camera.zoom.x = self.camera.zoom.y / screen_width() * screen_height();
        set_camera(&self.camera);
    }
}
