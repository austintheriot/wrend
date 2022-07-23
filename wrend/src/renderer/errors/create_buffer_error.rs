use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateBufferError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
}
