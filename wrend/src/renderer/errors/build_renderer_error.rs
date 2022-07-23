use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum BuildRendererError {
    #[error("No canvas was supplied")]
    NoCanvas,
    #[error("No WebGL2RenderingContext was supplied")]
    NoContext,
    #[error("No RenderCallback was supplied")]
    NoRenderCallback,
}
