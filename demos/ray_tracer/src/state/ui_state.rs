#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiState {
    is_keyboard_user: bool,
}

impl UiState {
    pub fn is_keyboard_user(&self) -> bool {
        self.is_keyboard_user
    }

    pub fn set_is_keyboard_user(&mut self) {
        self.is_keyboard_user = true;
    }
}
