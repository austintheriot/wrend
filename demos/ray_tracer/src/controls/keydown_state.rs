use super::keydown_key::KeydownKey;
use std::ops::{Index, IndexMut};

/// Keeps track of what keys are currently being held down
/// (used for updating the user's location in the world)
#[derive(Default, Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct KeydownState {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    space: bool,
    shift: bool,
}

impl KeydownState {
    pub fn no_keys_down(&self) -> bool {
        !self.w && !self.a && !self.s && !self.d && !self.space && !self.shift
    }

    pub fn set_all_false(&mut self) -> &mut Self {
        *self = Default::default();
        self
    }
}

impl Index<KeydownKey> for KeydownState {
    type Output = bool;

    fn index(&self, index: KeydownKey) -> &Self::Output {
        match index {
            KeydownKey::W => &self.w,
            KeydownKey::A => &self.a,
            KeydownKey::S => &self.s,
            KeydownKey::D => &self.d,
            KeydownKey::Space => &self.space,
            KeydownKey::Shift => &self.shift,
        }
    }
}

impl IndexMut<KeydownKey> for KeydownState {
    fn index_mut(&mut self, index: KeydownKey) -> &mut Self::Output {
        match index {
            KeydownKey::W => &mut self.w,
            KeydownKey::A => &mut self.a,
            KeydownKey::S => &mut self.s,
            KeydownKey::D => &mut self.d,
            KeydownKey::Space => &mut self.space,
            KeydownKey::Shift => &mut self.shift,
        }
    }
}
