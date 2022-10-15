#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiState {
    is_keyboard_user: bool,
    show_menu: bool,
    is_recording: bool,
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

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    pub fn set_is_recording(&mut self, is_recording: bool) -> &mut Self {
        self.is_recording = is_recording;
        self
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            is_keyboard_user: Default::default(),
            show_menu: true,
            is_recording: false,
        }
    }
}
