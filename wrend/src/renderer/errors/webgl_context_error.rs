use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum WebGlContextError {
    #[error(
        "Error occurred while trying to get a WebGL2 rendering context from the supplied canvas"
    )]
    RetrievalError,
    #[error("WebGL2 rendering context could not be acquired from the canvas. The returned value was `None`")]
    NotFoundError,
    #[error("The JavaScript Object returned from get_context could not be converted into a `WebGl2RenderingContext`")]
    TypeConversionError,
}