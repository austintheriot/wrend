use std::{any::Any, cell::RefCell, rc::Rc};

use wasm_bindgen::JsValue;

use crate::{
    AnimationCallback, Id, IdDefault, IdName, RendererData, RendererDataJs,
    RendererDataJsInner, Callback,
};
use log::error;

#[derive(Clone, Debug)]
pub(crate) struct AnimationData<
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
        &self,
        renderer_data: Rc<
            RefCell<
                RendererData<
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
        >,
    ) {
        if let Some(animation_callback) = &self.animation_callback {
            // if the types are compatible with JavaScript, treat as a special case and pass in the `RendererData` to the JavaScript function
            let rendered = if let Some(renderer_data) =
                (&renderer_data as &dyn Any).downcast_ref::<Rc<RefCell<RendererDataJsInner>>>()
            {
                let renderer_data = Rc::clone(renderer_data);
                match &**animation_callback {
                    Callback::Rust(_) => false,
                    Callback::Js(js_callback) => {
                        let renderer_data_js: RendererDataJs = renderer_data.into();
                        let js_value: JsValue = renderer_data_js.into();
                        let result = js_callback.call1(&JsValue::NULL, &js_value);
                        if let Err(err) = result {
                            error!("Error occurred while calling JavaScript animation callback: {err:?}");
                        }
                        true
                    },
                }
            } else {
                false
            };

            // if not already rendered in JavaScript, call with Rust values
            // this does not pass the `RendererData` to the JavaScript callback if one was supplied,
            // since the types are not compatible with the JavaScript/Wasm API
            if !rendered {
                animation_callback.call_with_rust_arg(&renderer_data.borrow());
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
