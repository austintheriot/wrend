#[derive(Debug, Clone)]
pub enum UiStateAction {
    SetIsKeyboardUser,
    SetShowMenu(bool),
    SetIsRecording(bool),
}
