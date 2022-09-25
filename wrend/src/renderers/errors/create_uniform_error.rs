use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateUniformError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("The associated program_id could no be found")]
    ProgramNotFound,
    #[error("The uniform's location was not found in the program: {uniform_id:?}")]
    UniformLocationNotFound { uniform_id: String, program_id: String },
}
