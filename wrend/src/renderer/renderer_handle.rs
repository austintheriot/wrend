use crate::{AnimationCallback, AnimationData, Id, IdName, Listener, RecordingData, Renderer};
use js_sys::{Array, ArrayBuffer, Uint8Array};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, BlobEvent, Blob, Url, HtmlAnchorElement, BlobPropertyBag};

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
        let handle_data_available = {
            let recording_data = recording_data.clone();
            Listener::new(
                media_recorder.clone(),
                "dataavailable",
                move |e: BlobEvent| {
                    info!("Data available! {:#?}", e.data());
                    if let Some(blob) = e.data() {
                        let recording_data = recording_data.clone();

                        wasm_bindgen_futures::spawn_local(async move {
                            let bytes_array_buffer: ArrayBuffer = JsFuture::from(
                                blob.array_buffer(),
                            )
                            .await
                            .expect("Should be able to get array buffer from recorded Blob data")
                            .dyn_into()
                            .unwrap();
                            let bytes_array = Uint8Array::new(bytes_array_buffer.as_ref());
                            let bytes = bytes_array.to_vec();
                            recording_data
                                .borrow_mut()
                                .recorded_chunks_mut()
                                .extend(bytes);

                            info!("Data successfully read from Blob and saved in wasm memory");
                        })
                    }
                },
            )
        };

        let handle_stop = {
            let recording_data = Rc::clone(&recording_data);
            Listener::new(media_recorder, "stop", move|e| {
                info!("Recording stopped! {:#?}", e);
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let body = document.body().unwrap();
                let a: HtmlAnchorElement =
                    document.create_element("a").unwrap().dyn_into().unwrap();
                a.style().set_css_text("display: none;");
                a.set_download("canvas.webm");
                body.append_child(&a).unwrap();
                let recorded_data_ref = recording_data.borrow();
                let recorded_chunks = recorded_data_ref.recorded_chunks().as_slice();

                // data must be passed to blob constructor inside of a javascript array
                let blob_parts = Array::new_with_length(1);

                // it is unsafe to get a raw view into WebAssembly memory, but because this memory gets immediately
                // used, downloaded, and then view is discarded, it is safe so long as no new allocations are
                // made in between acquiring the view and using it
                let uint8_array = unsafe { Uint8Array::view(recorded_chunks) };
                blob_parts.set(0, uint8_array.dyn_into().unwrap());

                let mut blob_property_bag = BlobPropertyBag::new();
                blob_property_bag.type_(RecordingData::VIDEO_TYPE);
                let blob = Blob::new_with_buffer_source_sequence_and_options(blob_parts.as_ref(), &blob_property_bag).unwrap();

                let url = Url::create_object_url_with_blob(&blob).unwrap();

                a.set_href(&url);
                a.click();

                // release url from window memory when done to prevent memory leak
                // (this does not get released automatically, unlike most of web memory)
                Url::revoke_object_url(&url).unwrap();
            })
        };

        recording_data
            .borrow_mut()
            .set_recording_data_available_listener(Some(handle_data_available));
        recording_data
            .borrow_mut()
            .set_recording_stop_listener(Some(handle_stop));

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
        self.recording_data
            .borrow()
            .media_recorder()
            .start_with_time_slice(1000)
            .expect("Should be able to start media recorder");
    }

    pub fn stop_recording(&self) {
        // @todo: Add some MediaRecorder state checks here and/or internal state checks here
        self.recording_data
            .borrow()
            .media_recorder()
            .stop()
            .expect("Should be able to stop media recorder")
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
        // make sure recording listeners get dropped
        self.recording_data
            .borrow_mut()
            .set_recording_data_available_listener(None);
        self.recording_data
            .borrow_mut()
            .set_recording_stop_listener(None);
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
