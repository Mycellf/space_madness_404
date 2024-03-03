use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Keybinds {
    mappings: HashMap<KeyAction, (Vec<KeyCode>, PressedState)>,
}

impl Keybinds {
    pub fn update(&mut self) {
        for entry in self.mappings.values_mut() {
            let key_codes = &entry.0;

            entry.1 = entry
                .1
                .update(key_codes.into_iter().any(|key_code| is_key_down(*key_code)));
        }
    }

    pub fn add_key(&mut self, action: KeyAction, key_code: KeyCode) {
        self.mappings
            .get_mut(&action)
            .expect("All KeyActions should be valid key mappings")
            .0
            .push(key_code);
    }

    pub fn get_keys(&self, action: KeyAction) -> &Vec<KeyCode> {
        &self
            .mappings
            .get(&action)
            .expect("All KeyActions should be valid key mappings")
            .0
    }

    pub fn get_keys_mut(&mut self, action: KeyAction) -> &mut Vec<KeyCode> {
        &mut self
            .mappings
            .get_mut(&action)
            .expect("All KeyActions should be valid key mappings")
            .0
    }

    pub fn get(&self, action: KeyAction) -> PressedState {
        self.mappings
            .get(&action)
            .expect("All KeyActions should be valid key mappings")
            .1
    }

    pub fn get_keybinds(&self) -> &HashMap<KeyAction, (Vec<KeyCode>, PressedState)> {
        &self.mappings
    }

    fn default_map() -> HashMap<KeyAction, (Vec<KeyCode>, PressedState)> {
        use PressedState::Off;

        let mut map = HashMap::new();

        map.insert(KeyAction::Boost, (vec![KeyCode::W, KeyCode::Up], Off));
        map.insert(KeyAction::Pause, (vec![KeyCode::Escape], Off));

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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum KeyAction {
    Boost,
    Pause,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PressedState {
    JustPressed,
    Pressed,
    Off,
}

impl PressedState {
    pub fn is_just_pressed(self) -> bool {
        match self {
            Self::JustPressed => true,
            Self::Pressed => false,
            Self::Off => false,
        }
    }

    pub fn is_pressed(self) -> bool {
        match self {
            Self::JustPressed => true,
            Self::Pressed => true,
            Self::Off => false,
        }
    }

    pub fn is_not_pressed(self) -> bool {
        !self.is_pressed()
    }

    pub fn update(self, state: bool) -> Self {
        match (self, state) {
            (Self::Off, true) => Self::JustPressed,
            (_, true) => Self::Pressed,
            (_, false) => Self::Off,
        }
    }
}
