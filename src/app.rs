use macroquad::prelude::*;

pub struct App {
    fixed_tick_time: f32,
    camera: Camera2D,
}

impl App {
    pub const FIXED_TICKS_PER_SEC: f32 = 60.0;
    pub const MAX_TICKS_PER_FRAME: u32 = 5;

    pub fn new() -> Self {
        Self {
            fixed_tick_time: 0.0,
            camera: Camera2D {
                zoom: Vec2::splat(1.0 / 128.0),
                ..Default::default()
            },
        }
    }

    pub fn frame_tick(&mut self) {
        self.update_camera();

        clear_background(BLACK);
    }

    pub fn check_fixed_tick(&mut self) {
        self.fixed_tick_time += get_frame_time() * Self::FIXED_TICKS_PER_SEC;

        for _ in 0..(self.fixed_tick_time as u32).min(Self::MAX_TICKS_PER_FRAME) {
            self.fixed_tick();
        }

        self.fixed_tick_time %= 1.0;
    }

    pub fn fixed_tick(&mut self) {
        self.camera.target.x += 0.1;
    }

    fn update_camera(&mut self) {
        self.camera.zoom.x = self.camera.zoom.y / screen_width() * screen_height();
        set_camera(&self.camera);
    }
}
