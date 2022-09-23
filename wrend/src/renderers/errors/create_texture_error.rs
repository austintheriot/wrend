use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateTextureError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("No Canvas was provided")]
    NoCanvas,
}
