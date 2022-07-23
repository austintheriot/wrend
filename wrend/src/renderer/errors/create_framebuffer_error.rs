use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateFramebufferError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
}
