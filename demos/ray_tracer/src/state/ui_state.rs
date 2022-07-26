#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiState {
    is_keyboard_user: bool,
    show_menu: bool,
}

impl UiState {
    pub fn is_keyboard_user(&self) -> bool {
        self.is_keyboard_user
    }

    pub fn set_is_keyboard_user(&mut self) -> &mut Self {
        self.is_keyboard_user = true;
        self
    }

    pub fn show_menu(&self) -> bool {
        self.show_menu
    }

    pub fn set_show_menu(&mut self, show_menu: bool) -> &mut Self {
        self.show_menu = show_menu;
        self
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            is_keyboard_user: Default::default(),
            show_menu: true,
        }
    }
}
