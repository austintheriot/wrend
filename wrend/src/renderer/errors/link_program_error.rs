use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkProgramError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("No vertex shader was found associated with the id provided")]
    VertexShaderNotFound,
    #[error("No fragment shader was found associated with the id provided")]
    FragmentShaderNotFound,
    #[error("ProgramLink could not be found for ProgramId provided")]
    NoProgramLink,
    #[error("Value returned by `gl.link_program` was `None`")]
    NoProgram,
    #[error("{0}")]
    KnownError(String),
    #[error("Varyings could not be converted into a JavaScript array")]
    CouldNotConvertVaryingsToArray,
    #[error("An unknown error occurred")]
    UnknownError,
}