use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Keybinds {
    mappings: HashMap<KeyAction, (Vec<KeyCode>, bool)>,
}

impl Keybinds {
    pub fn add_keybind(&mut self, action: KeyAction, key_code: KeyCode) -> Option<()> {
        self.mappings.get_mut(&action)?.0.push(key_code);
        Some(())
    }

    pub fn get_keys(&self, action: KeyAction) -> Option<&Vec<KeyCode>> {
        Some(&self.mappings.get(&action)?.0)
    }

    pub fn get_keys_mut(&mut self, action: KeyAction) -> Option<&mut Vec<KeyCode>> {
        Some(&mut self.mappings.get_mut(&action)?.0)
    }

    pub fn get_pressed(&self, action: KeyAction) -> Option<bool> {
        Some(self.mappings.get(&action)?.1)
    }

    pub fn get_keybinds(&self) -> &HashMap<KeyAction, (Vec<KeyCode>, bool)> {
        &self.mappings
    }

    fn default_map() -> HashMap<KeyAction, (Vec<KeyCode>, bool)> {
        let mut map = HashMap::new();

        map.insert(KeyAction::Boost, (vec![KeyCode::W, KeyCode::Up], false));
        map.insert(KeyAction::Pause, (vec![KeyCode::Escape], false));

        map
    }
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            mappings: Self::default_map(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyAction {
    Boost,
    Pause,
}
