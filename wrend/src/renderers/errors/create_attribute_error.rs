use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateAttributeError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("Attribute link's associated program was not found from the program_id")]
    ProgramNotFound,
    #[error("Attribute link's associated Vertex Array Object was not found from the program_id")]
    VAONotFound,
    #[error("Attribute link's associated buffer was not found from the buffer_id")]
    BufferNotFound,
    #[error("Attribute link's associated location was not found")]
    AttributeLocationNotFound,
}
