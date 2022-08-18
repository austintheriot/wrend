use crate::recording::recording_data;
use crate::{AnimationHandle, Id, IdName, RecordingData};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::BlobEvent;

fn test_handle_data_available(e: BlobEvent) {
    info!("Data available! {:#?}", e.data());
}

#[derive(Clone)]
pub struct RecordingHandle<
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
>(
    Rc<
        RefCell<
            RecordingData<
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
);

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
    RecordingHandle<
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
        recording_data: RecordingData<
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
        RecordingHandle(Rc::new(RefCell::new(recording_data)))
    }

    pub fn start_recording(&self) {
        todo!("Add some MediaRecorder state checks here and/or internal state checks here");
        self.0
            .borrow()
            .media_recorder()
            .start_with_time_slice(1000)
            .expect("Should be able to start media recorder");
    }

    pub fn stop_recording(&self) {
        todo!("Add some MediaRecorder state checks here and/or internal state checks here");
        self.0
            .borrow()
            .media_recorder()
            .stop()
            .expect("Should be able to stop media recorder")
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
        AnimationHandle<
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
    for RecordingHandle<
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
        animation_handle: AnimationHandle<
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
        let mut recording_data: RecordingData<
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
        > = animation_handle.into();

        // testing: receiving data through data_available event
        let handle_data_available =
            Closure::wrap(Box::new(test_handle_data_available) as Box<dyn FnMut(BlobEvent)>);
        recording_data
            .media_recorder_mut()
            .set_ondataavailable(Some(handle_data_available.as_ref().unchecked_ref()));
        handle_data_available.forget();

        Self::new(recording_data)
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
    for RecordingHandle<
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
        todo!("Make sure data available callbacks and others are destroyed correctly");

        self.stop_recording();
    }
}
