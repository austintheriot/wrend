use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateTransformFeedbackError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("The value returned from `create_transform_feedback` was None")]
    NoneWasReturned,
}