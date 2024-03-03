use crate::app::App;
use crate::keybinds::KeyAction;
use crate::object::Object;
use macroquad::{miniquad::window::screen_size, prelude::*};
use nalgebra::{Complex, Unit};
use rapier2d::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Component {
    CameraFollow,
    Motion { power: f32, brake: f32 },
    FaceMouse,
}

impl Component {
    pub fn fixed_update(self, object: &mut Object, app: &mut App) {
        match self {
            Self::CameraFollow => {}
            Self::Motion { power, brake } => {
                if app.keybinds.get(KeyAction::Boost).is_pressed() {
                    let rigid_body = app.get_rigid_body_mut(object);
                    let rotation = rigid_body.rotation();
                    let rotation = vector![rotation.re, rotation.im];
                    rigid_body.apply_impulse(rotation * power, true);
                }
                if app.keybinds.get(KeyAction::Slow).is_pressed() {
                    let rigid_body = app.get_rigid_body_mut(object);
                    rigid_body.set_linvel(rigid_body.linvel() * brake, true);
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

    pub fn frame_update(self, object: &mut Object, app: &mut App) {
        match self {
            Self::CameraFollow => {
                app.camera.target = (*app.get_rigid_body(object).center_of_mass()).into();
            }
            Self::Motion { power: _, brake: _ } => {}
            Self::FaceMouse => {}
        }
    }
}
