use crate::{Listener, RecordingData};
use js_sys::{ArrayBuffer, Uint8Array};
use log::{error, info};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{BlobEvent, Event, MediaRecorder, MediaRecorderErrorEvent};

pub fn make_handle_start(
    media_recorder: MediaRecorder,
    recording_data: Rc<RefCell<RecordingData>>,
) -> Listener<MediaRecorder, Event> {
    Listener::new(media_recorder, "start", move |_: Event| {
        info!("Recording started");
        recording_data.borrow_mut().set_is_recording(true);
    })
}

pub fn make_handle_stop(
    media_recorder: MediaRecorder,
    recording_data: Rc<RefCell<RecordingData>>,
) -> Listener<MediaRecorder, Event> {
    Listener::new(media_recorder, "stop", move |_: Event| {
        info!("Recording stopped");
        recording_data.borrow_mut().set_is_recording(false);
    })
}

pub fn make_handle_dataavailable(
    media_recorder: MediaRecorder,
    recording_data: Rc<RefCell<RecordingData>>,
) -> Listener<MediaRecorder, BlobEvent> {
    Listener::new(media_recorder, "dataavailable", move |e: BlobEvent| {
        info!("Recording data available");
        if let Some(blob) = e.data() {
            let recording_data = recording_data.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let bytes_array_buffer: ArrayBuffer = JsFuture::from(blob.array_buffer())
                    .await
                    .expect("Should be able to get array buffer from recorded Blob data")
                    .dyn_into()
                    .expect("Should be able to interpret JsValue as an ArrayBuffer");
                let bytes_array = Uint8Array::new(bytes_array_buffer.as_ref());
                let bytes = bytes_array.to_vec();
                recording_data
                    .borrow_mut()
                    .recorded_chunks_mut()
                    .extend(bytes);

                // intuitively, it would make the most sense to download the video in the stop handler rather than here,
                // but some (all?) browsers emit the `stop` event BEFORE the `dataavailable` event, which
                // means that some data can accidentally be omitted if the file is downloaded before this final
                // `dataavailable` event was emitted after a `stop`.
                if !recording_data.borrow().is_recording() {
                    recording_data.borrow().download_video();
                }
            })
        }
    })
}

pub fn make_handle_error(
    media_recorder: MediaRecorder,
    recording_data: Rc<RefCell<RecordingData>>,
) -> Listener<MediaRecorder, MediaRecorderErrorEvent> {
    Listener::new(
        media_recorder,
        "error",
        move |e: MediaRecorderErrorEvent| {
            error!("Error occurred while recording video: {:?}", e);
            recording_data.borrow_mut().set_is_recording(false);
        },
    )
}

pub fn make_handle_pause(
    media_recorder: MediaRecorder,
    recording_data: Rc<RefCell<RecordingData>>,
) -> Listener<MediaRecorder, Event> {
    Listener::new(media_recorder, "stop", move |_: Event| {
        info!("Recording paused");
        recording_data.borrow_mut().set_is_recording(false);
    })
}

pub fn make_handle_resume(
    media_recorder: MediaRecorder,
    recording_data: Rc<RefCell<RecordingData>>,
) -> Listener<MediaRecorder, Event> {
    Listener::new(media_recorder, "stop", move |_: Event| {
        info!("Recording resumed");
        recording_data.borrow_mut().set_is_recording(false);
    })
}
