use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CompileShaderError {
    #[error("No canvas or its associated context were supplied")]
    NoContext,
    #[error("Call to WebGL2RenderingContext returned None")]
    NoShaderReturned,
    #[error("{0}")]
    KnownError(String),
    #[error("An unknown error occurred.")]
    UnknownError,
}
