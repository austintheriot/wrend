use crate::{
    recording_handlers, AnimationCallback, AnimationData, Id, IdName, RecordingData, Renderer,
};

use log::warn;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::window;

/// The `RendererHandle` struct takes ownership of the `Renderer`, enabling it to
/// perform more complex operations than would otherwise be possible, such as
/// animating renders over time or recording canvas output.
#[derive(Clone)]
pub struct RendererHandle<
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
> {
    renderer: Rc<
        RefCell<
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
                VertexArrayObjectId,
                UserCtx,
            >,
        >,
    >,
    animation_data: Rc<
        RefCell<
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
            >,
        >,
    >,
    recording_data: Rc<RefCell<RecordingData>>,
}

impl<
        VertexShaderId: 'static + Id,
        FragmentShaderId: 'static + Id,
        ProgramId: 'static + Id,
        UniformId: 'static + Id + IdName,
        BufferId: 'static + Id,
        AttributeId: 'static + Id + IdName,
        TextureId: 'static + Id,
        FramebufferId: 'static + Id,
        TransformFeedbackId: 'static + Id,
        VertexArrayObjectId: 'static + Id,
        UserCtx: Clone + 'static,
    >
    RendererHandle<
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
    pub fn new(
        renderer: Renderer<
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
        let recording_data = RecordingData::new(&renderer);
        let media_recorder = recording_data.media_recorder().clone();
        let recording_data = Rc::new(RefCell::new(recording_data));

        {
            let mut recording_data_ref = recording_data.borrow_mut();
            recording_data_ref
                .add_event_listener(recording_handlers::make_handle_dataavailable(
                    media_recorder.clone(),
                    Rc::clone(&recording_data),
                ))
                .add_event_listener(recording_handlers::make_handle_start(
                    media_recorder.clone(),
                    Rc::clone(&recording_data),
                ))
                .add_event_listener(recording_handlers::make_handle_error(
                    media_recorder.clone(),
                    Rc::clone(&recording_data),
                ))
                .add_event_listener(recording_handlers::make_handle_stop(
                    media_recorder.clone(),
                    Rc::clone(&recording_data),
                ))
                .add_event_listener(recording_handlers::make_handle_pause(
                    media_recorder.clone(),
                    Rc::clone(&recording_data),
                ))
                .add_event_listener(recording_handlers::make_handle_pause(
                    media_recorder,
                    Rc::clone(&recording_data),
                ));
        }

        Self {
            recording_data,
            renderer: Rc::new(RefCell::new(renderer)),
            animation_data: Rc::new(RefCell::new(AnimationData::new())),
        }
    }

    pub fn start_animating(&self) {
        // cancel previous animation before starting a new one
        self.stop_animating();

        self.animation_data.borrow_mut().set_is_animating(true);
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let animation_data = Rc::clone(&self.animation_data);
        let renderer = Rc::clone(&self.renderer);
        {
            let animation_data = Rc::clone(&self.animation_data);
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                // do not run callback if not animating
                if !animation_data.borrow().is_animating() {
                    return;
                }

                // run animation callback
                animation_data
                    .borrow_mut()
                    .call_animation_callback(&mut renderer.borrow_mut());

                // schedule another requestAnimationFrame callback
                let animation_id = Self::request_animation_frame(f.borrow().as_ref().unwrap());
                animation_data.borrow_mut().set_request_id(animation_id);
            }) as Box<dyn Fn()>));
        }

        let id = Self::request_animation_frame(g.borrow().as_ref().unwrap());
        animation_data.borrow_mut().set_request_id(id);
    }

    pub fn stop_animating(&self) {
        self.animation_data.borrow_mut().set_is_animating(false);
        window()
            .unwrap()
            .cancel_animation_frame(self.animation_data.borrow().request_id())
            .expect("Should be able to cancel animation frame")
    }

    pub fn set_animation_callback(
        &mut self,
        animation_callback: Option<
            impl Into<
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
        >,
    ) {
        self.animation_data
            .borrow_mut()
            .set_animation_callback(animation_callback.map(|cb| cb.into()));
    }

    pub fn start_recording(&self) {
        // @todo: Add some MediaRecorder state checks here and/or internal state checks here
        if let Err(err) = self
            .recording_data
            .borrow()
            .media_recorder()
            .start_with_time_slice(RecordingData::SAVE_DATA_INTERVAL)
        {
            warn!("Error trying to start video recording: {:?}", err);
        }
    }

    pub fn stop_recording(&self) {
        // @todo: Add some MediaRecorder state checks here and/or internal state checks here
        if let Err(err) = self.recording_data.borrow().media_recorder().stop() {
            warn!("Error trying to stop video recording: {:?}", err);
        }
    }

    fn request_animation_frame(f: &Closure<dyn Fn()>) -> i32 {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` ok")
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
        UserCtx: Clone,
    > Drop
    for RendererHandle<
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
    fn drop(&mut self) {
        self.recording_data
            .borrow_mut()
            .remove_all_event_listeners();
        self.stop_recording();
        self.stop_animating();
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
        UserCtx: Clone,
    >
    From<
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
            VertexArrayObjectId,
            UserCtx,
        >,
    >
    for RendererHandle<
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
    fn from(
        renderer: Renderer<
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
        RendererHandle::new(renderer)
    }
}
