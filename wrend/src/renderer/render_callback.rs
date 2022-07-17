use crate::renderer::renderer::Renderer;
use crate::CallbackWithContext;

pub type RenderCallback<
    VertexShaderId,
    FragmentShaderId,
    ProgramId,
    UniformId,
    BufferId,
    AttributeId,
    TextureId,
    FramebufferId,
    UserCtx,
> = CallbackWithContext<
    Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        UserCtx,
    >,
>;
