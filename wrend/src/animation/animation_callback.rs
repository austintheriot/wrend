use crate::{CallbackWithContext, Renderer};

pub type AnimationCallback<
    VertexShaderId,
    FragmentShaderId,
    ProgramId,
    UniformId,
    BufferId,
    AttributeId,
    TextureId,
    FramebufferId,
    TransformFeedbackId,
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
        TransformFeedbackId,
        UserCtx,
    >,
>;
