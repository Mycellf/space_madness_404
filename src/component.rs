use crate::app::App;
use crate::keybinds::KeyAction;
use crate::object::Object;
use macroquad::{miniquad::window::screen_size, prelude::*};
use nalgebra::{Complex, Unit};
use rapier2d::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Component {
    CameraFollow,
    Motion {
        power: f32,
        brake: f32,
        emitter: Vec2,
    },
    FaceMouse,
}

impl Component {
    /// Occurs during the fixed update, just before the physics_update
    /// is called for a given object.
    pub fn fixed_update(&mut self, object: &mut Object, app: &mut App) {
        match self {
            Self::CameraFollow => {}
            Self::Motion {
                power: _,
                brake: _,
                emitter: _,
            } => {}
            Self::FaceMouse => {}
        }
    }

    /// Occurs when the game is not paused, during the fixed
    /// timestep, after the fixed update is called for a given
    /// object.
    pub fn physics_update(&mut self, object: &mut Object, app: &mut App) {
        match self {
            Self::CameraFollow => {}
            Self::Motion {
                power,
                brake,
                emitter: _,
            } => {
                if app.keybinds.get(KeyAction::Boost).is_pressed() {
                    let rigid_body = app.get_rigid_body_mut(object);
                    let rotation = rigid_body.rotation();
                    let rotation = vector![rotation.re, rotation.im];
                    rigid_body.apply_impulse(rotation * *power, true);
                }
                if app.keybinds.get(KeyAction::Slow).is_pressed() {
                    let rigid_body = app.get_rigid_body_mut(object);
                    rigid_body.set_linvel(rigid_body.linvel() * *brake, true);
                }
            }
            Self::FaceMouse => {
                let rigid_body = app.get_rigid_body_mut(object);
                let mouse = Vec2::from(mouse_position_local()) * Vec2::from(screen_size());
                let target = Complex::<f32>::new(mouse.x, mouse.y);
                let target = Unit::<Complex<f32>>::new_normalize(target);
                let angle = rigid_body.rotation().angle_to(&target) * std::f32::consts::PI;
                if angle.is_finite() {
                    let inertia = rigid_body.mass_properties().effective_angular_inertia();

                    let velocity = rigid_body.angvel();
                    let distance = angle.abs();

                    let coefficient = inertia / (distance + 1.0);

                    rigid_body.apply_torque_impulse((angle - velocity) * coefficient, true);
                }
            }
        }
    }

    /// Occurs before each frame is rendered, after the fixed and
    /// physics updates are called.
    pub fn frame_update(&mut self, object: &mut Object, app: &mut App) {
        match self {
            Self::CameraFollow => {
                app.camera.target = (*app.get_rigid_body(object).center_of_mass()).into();
            }
            Self::Motion {
                power: _,
                brake: _,
                emitter: _,
            } => {}
            Self::FaceMouse => {}
        }
    }

    pub fn draw(&self, object: &Object, app: &App) {
        match self {
            Self::CameraFollow => {}
            Self::Motion {
                power: _,
                brake: _,
                emitter,
            } => {
                const UP: Vec2 = Vec2::new(0.0, 2.0);
                const LEFT: Vec2 = Vec2::new(0.05, 0.0);

                if !app.keybinds.get(KeyAction::Boost).is_pressed() {
                    return;
                }

                let rigod_body = app.get_rigid_body(object);
                let position = rigod_body.position();
                let a = position.transform_point(&(*emitter + UP - LEFT).into());
                let b = position.transform_point(&(*emitter - UP - LEFT).into());

                gl_use_default_material();
                draw_line(a.x, a.y, b.x, b.y, 0.1, WHITE);
            }
            Self::FaceMouse => {}
        }
    }
}
