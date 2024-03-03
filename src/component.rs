use crate::app::App;
use crate::keybinds::KeyAction;
use crate::object::Object;
use rapier2d::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Component {
    CameraFollow,
    Boost { power: f32 },
    FaceMouse,
}

impl Component {
    pub fn fixed_update(self, object: &mut Object, app: &mut App) {
        match self {
            Self::CameraFollow => {}
            Self::Boost { power } => {
                if app.keybinds.get(KeyAction::Boost).is_pressed() {
                    let rigid_body = app.get_rigid_body_mut(object);
                    let rotation = rigid_body.rotation();
                    let rotation = vector![rotation.re, rotation.im];
                    rigid_body.apply_impulse(rotation * power, true);
                }
            }
            Self::FaceMouse => {}
        }
    }

    pub fn frame_update(self, object: &mut Object, app: &mut App) {
        match self {
            Self::CameraFollow => {
                app.camera.target = (*app.get_rigid_body(object).center_of_mass()).into();
            }
            Self::Boost { power: _ } => {}
            Self::FaceMouse => {}
        }
    }
}
