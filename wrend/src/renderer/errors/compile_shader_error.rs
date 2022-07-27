use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CompileShaderError {
    #[error("{shader_id:?}: No canvas or its associated context were supplied")]
    NoContext { shader_id: String },
    #[error("{shader_id:?}: Call to WebGL2RenderingContext returned None")]
    NoShaderReturned { shader_id: String },
    #[error("{shader_id:?}: {error:?}")]
    KnownError { shader_id: String, error: String },
    #[error("{shader_id:?}: An unknown error occurred.")]
    UnknownError { shader_id: String },
}
