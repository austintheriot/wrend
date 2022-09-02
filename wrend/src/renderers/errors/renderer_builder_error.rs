use crate::{
    BuildRendererError, CompileShaderError, CreateAttributeError, CreateBufferError,
    CreateFramebufferError, CreateTextureError, CreateTransformFeedbackError, CreateUniformError,
    CreateVAOError, LinkProgramError, SaveContextError, WebGlContextError,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum RendererBuilderError {
    #[error("Error occurred while retrieving the WebGL2 context: {0:?}")]
    WebGlContextError(#[from] WebGlContextError),
    #[error("Error occurred while building the RendererData {0:?}")]
    RendererBuildError(#[from] BuildRendererError),
    #[error("Error occurred while compiling shader: {0:?}")]
    CompileShaderError(#[from] CompileShaderError),
    #[error("Error occurred while linking program: {0:?}")]
    LinkProgramError(#[from] LinkProgramError),
    #[error("Error occurred while initializing uniforms: {0:?}")]
    UniformError(#[from] CreateUniformError),
    #[error("Error occurred while trying to retrieve WebGL context from canvas: {0:?}")]
    SaveContextError(#[from] SaveContextError),
    #[error("Error occurred while trying to create Vertex Array Object: {0:?}")]
    CreateVAOError(#[from] CreateVAOError),
    #[error("Error occurred while trying to initialize attribute: {0:?}")]
    InitializeAttributeError(#[from] CreateAttributeError),
    #[error("Error occurred while trying to create buffer: {0:?}")]
    CreateBufferError(#[from] CreateBufferError),
    #[error("Error occurred while trying to create texture: {0:?}")]
    CreateTextureError(#[from] CreateTextureError),
    #[error("Error occurred while trying to create framebuffer: {0:?}")]
    CreateFramebufferError(#[from] CreateFramebufferError),
    #[error("Error occurred while trying to create transform feedback: {0:?}")]
    CreateTransformFeedbackError(#[from] CreateTransformFeedbackError),
}
