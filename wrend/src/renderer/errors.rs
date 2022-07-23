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

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum BuildRendererError {
    #[error("No canvas was supplied")]
    NoCanvas,
    #[error("No WebGL2RenderingContext was supplied")]
    NoContext,
    #[error("No RenderCallback was supplied")]
    NoRenderCallback,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CompileShaderError {
    #[error("No canvas or its associated context were supplied")]
    NoContext,
    #[error("Call to WebGL2RenderingContext returned None")]
    NoShaderReturned,
    #[error("{0}")]
    KnownError(String),
    #[error("An unknown error occurred.")]
    UnknownError,
}

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

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateUniformError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("The associated program_id could no be found")]
    ProgramNotFound,
    #[error("The uniform's location was not found in the program: {uniform_id:?}")]
    UniformLocationNotFound { uniform_id: String },
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum SaveContextError {
    #[error("`None` was returned")]
    CanvasReturnedNoContext,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateVAOError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("The VAO returned from the WebGL2 context was None")]
    NoneWasReturned,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum InitializeAttributeError {
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

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateBufferError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateTextureError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateFramebufferError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CreateTransformFeedbackError {
    #[error("No WebGL2RenderingContext was provided")]
    NoContext,
    #[error("The value returned from `create_transform_feedback` was None")]
    NoneWasReturned,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum RendererBuilderError {
    #[error("Error occurred while retrieving the WebGL2 context: {0}")]
    WebGlContextError(#[from] WebGlContextError),
    #[error("Error occurred while building the Renderer {0}")]
    RendererBuildError(#[from] BuildRendererError),
    #[error("Error occurred while compiling shader: {0}")]
    CompileShaderError(#[from] CompileShaderError),
    #[error("Error occurred while linking program: {0}")]
    LinkProgramError(#[from] LinkProgramError),
    #[error("Error occurred while initializing uniforms: {0}")]
    UniformError(#[from] CreateUniformError),
    #[error("Error occurred while trying to retrieve WebGL context from canvas: {0}")]
    SaveContextError(#[from] SaveContextError),
    #[error("Error occurred while trying to create Vertex Array Object: {0}")]
    CreateVAOError(#[from] CreateVAOError),
    #[error("Error occurred while trying to initialize attribute: {0}")]
    InitializeAttributeError(#[from] InitializeAttributeError),
    #[error("Error occurred while trying to create buffer: {0}")]
    CreateBufferError(#[from] CreateBufferError),
    #[error("Error occurred while trying to create texture: {0}")]
    CreateTextureError(#[from] CreateTextureError),
    #[error("Error occurred while trying to create framebuffer: {0}")]
    CreateFramebufferError(#[from] CreateFramebufferError),
    #[error("Error occurred while trying to create transform feedback: {0}")]
    CreateTransformFeedbackError(#[from] CreateTransformFeedbackError),
}
