use std::ops::Deref;

use wasm_bindgen::JsValue;

use crate::{AnimationCallback, Either, Id, IdDefault, IdName, Renderer};

#[derive(Clone, Debug)]
pub struct AnimationData<
    VertexShaderId: Id = IdDefault,
    FragmentShaderId: Id = IdDefault,
    ProgramId: Id = IdDefault,
    UniformId: Id + IdName = IdDefault,
    BufferId: Id = IdDefault,
    AttributeId: Id + IdName = IdDefault,
    TextureId: Id = IdDefault,
    FramebufferId: Id = IdDefault,
    TransformFeedbackId: Id = IdDefault,
    VertexArrayObjectId: Id = IdDefault,
    UserCtx: Clone + 'static = (),
> {
    request_id: i32,
    animation_callback: Option<
        AnimationCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            AttributeId,
            TextureId,
            FramebufferId,
            TransformFeedbackId,
            VertexArrayObjectId,
            UserCtx,
        >,
    >,
    is_animating: bool,
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone + 'static,
    >
    AnimationData<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_animation_callback(
        animation_callback: AnimationCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            AttributeId,
            TextureId,
            FramebufferId,
            TransformFeedbackId,
            VertexArrayObjectId,
            UserCtx,
        >,
    ) -> Self {
        let mut animation_data = Self::default();
        animation_data.set_animation_callback(Some(animation_callback));
        animation_data
    }

    pub fn set_request_id(&mut self, id: i32) {
        self.request_id = id;
    }

    pub fn request_id(&self) -> i32 {
        self.request_id
    }

    /// Calls the internal animation callback.
    ///
    /// If no animation has been supplied yet, this is a no-op.
    pub fn call_animation_callback(
        &mut self,
        renderer: &mut Renderer<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            AttributeId,
            TextureId,
            FramebufferId,
            TransformFeedbackId,
            VertexArrayObjectId,
            UserCtx,
        >,
    ) {
        if let Some(animation_callback) = &self.animation_callback {
            match animation_callback.deref() {
                Either::A(rust_callback) => (rust_callback)(renderer),
                Either::B(js_callback) => {
                    let this = JsValue::NULL;
                    js_callback
                        .call0(&this)
                        .expect("Should be able to call animation callback");
                }
            }
        }
    }

    pub fn set_animation_callback(
        &mut self,
        animation_callback: Option<
            AnimationCallback<
                VertexShaderId,
                FragmentShaderId,
                ProgramId,
                UniformId,
                BufferId,
                AttributeId,
                TextureId,
                FramebufferId,
                TransformFeedbackId,
                VertexArrayObjectId,
                UserCtx,
            >,
        >,
    ) {
        self.animation_callback = animation_callback;
    }

    pub fn set_is_animating(&mut self, is_animating: bool) -> &mut Self {
        self.is_animating = is_animating;
        self
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone + 'static,
    > Default
    for AnimationData<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    fn default() -> Self {
        Self {
            animation_callback: None,
            // The `requestAnimationFrame` function is guaranteed to return a non-zero function,
            // so using an initial value of `0` here is guaranteed to be safe if it is accidentally
            // used to cancel a requested animation frame.
            request_id: 0,
            is_animating: false,
        }
    }
}
