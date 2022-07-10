use super::app_state::AppState;

pub trait AppSelector {}

impl AppSelector for AppState {}
