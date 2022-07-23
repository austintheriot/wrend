use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum SaveContextError {
    #[error("`None` was returned")]
    CanvasReturnedNoContext,
}