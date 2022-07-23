use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateVAOError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("The VAO returned from the WebGL2 context was None")]
    NoneWasReturned,
}
