use crate::app::App;
use crate::keybinds::KeyAction;
use crate::object::Object;
use rapier2d::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Component {
    CameraFollow,
    Boost,
    FaceMouse,
}

impl Component {
    pub fn fixed_update(self, object: &mut Object, app: &mut App) {
        match self {
            Self::CameraFollow => {}
            Self::Boost => {
                if app.keybinds.get(KeyAction::Boost).is_pressed() {
                    let rigid_body = &mut app.physics_world.rigid_body_set[object.rigid_body];
                    let rotation = rigid_body.rotation();
                    let rotation = vector![rotation.re, rotation.im];
                    rigid_body.apply_impulse(rotation * 100.0, true);
                }
            }
            Self::FaceMouse => {}
        }
    }
}
