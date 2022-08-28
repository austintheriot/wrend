use crate::Listener;
use js_sys::{Array, Uint8Array};
use log::info;
use std::{any::Any, ops::Deref};
use wasm_bindgen::{convert::FromWasmAbi, prelude::wasm_bindgen, JsCast};
use web_sys::{
    Blob, BlobPropertyBag, EventTarget, HtmlAnchorElement, HtmlCanvasElement, MediaRecorder,
    MediaRecorderOptions, MediaStream, Url,
};

#[wasm_bindgen(module = "/src/recording/captureStream.js")]
extern "C" {
    fn captureStreamFromCanvas(canvas: HtmlCanvasElement) -> MediaStream;
}

#[derive(Debug)]
pub(crate) struct RecordingData {
    recorded_chunks: Vec<u8>,
    media_recorder: MediaRecorder,
    /// It is not necessary to interact with this data after it is stored.
    /// It is only necessary to store the Listener, which removes event listeners when it is dropped
    listeners: Vec<Box<dyn Any>>,
    is_recording: bool,
}

impl RecordingData {
    /// How often (in ms) dataavailable events should be sent to record video
    pub const SAVE_DATA_INTERVAL: i32 = 1000;
    pub const VIDEO_TYPE: &'static str = "video/webm";

    /// Creates a `MediaStream` and `MediaRecorder` that is ready to being recording video
    /// from the canvas.
    pub fn new(canvas: impl AsRef<HtmlCanvasElement>) -> Self {
        let canvas = canvas.as_ref();
        let media_stream = captureStreamFromCanvas(canvas.clone());

        // see https://developer.mozilla.org/en-US/docs/Web/Media/Formats/Video_codecs#codec_details
        // this Codec is not ideal, but it's one of the few that's broadly supported by both Chrome and Firefox
        let mut media_recorder_options = MediaRecorderOptions::new();
        let mime_type_vp9 = format!("{}; codecs=vp9", Self::VIDEO_TYPE);
        let mime_type = if MediaRecorder::is_type_supported(&mime_type_vp9) {
            mime_type_vp9
        } else {
            format!("{}; codecs=vp8", Self::VIDEO_TYPE)
        };
        media_recorder_options.mime_type(&mime_type);
        media_recorder_options.bits_per_second(u32::MAX);

        let media_recorder = MediaRecorder::new_with_media_stream_and_media_recorder_options(
            &media_stream,
            &media_recorder_options,
        )
        .expect("Should be able to build media recorder");

        info!("Using mimeType: {:?}", media_recorder.mime_type());

        Self {
            media_recorder,
            recorded_chunks: Vec::new(),
            listeners: Vec::new(),
            is_recording: false,
        }
    }

    pub fn download_video(&self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
        a.style().set_css_text("display: none;");
        a.set_download("canvas.webm");
        body.append_child(&a).unwrap();
        let recorded_chunks = self.recorded_chunks().as_slice();

        // data must be passed to blob constructor inside of a javascript array
        let blob_parts = Array::new_with_length(1);

        // it is unsafe to get a raw view into WebAssembly memory, but because this memory gets immediately
        // used, downloaded, and then view is discarded, it is safe so long as no new allocations are
        // made in between acquiring the view and using it
        let uint8_array = unsafe { Uint8Array::view(recorded_chunks) };
        blob_parts.set(0, uint8_array.dyn_into().unwrap());

        let mut blob_property_bag = BlobPropertyBag::new();
        blob_property_bag.type_(RecordingData::VIDEO_TYPE);
        let blob = Blob::new_with_buffer_source_sequence_and_options(
            blob_parts.as_ref(),
            &blob_property_bag,
        )
        .unwrap();

        let url = Url::create_object_url_with_blob(&blob).unwrap();

        a.set_href(&url);
        a.click();

        // release url from window memory when done to prevent memory leak
        // (this does not get released automatically, unlike most of web memory)
        Url::revoke_object_url(&url).unwrap();
    }

    pub fn add_event_listener<
        Element: Deref<Target = EventTarget> + 'static,
        Arg: FromWasmAbi + 'static,
    >(
        &mut self,
        listener: Listener<Element, Arg>,
    ) -> &mut Self {
        self.listeners.push(Box::new(listener));
        self
    }

    pub fn remove_all_event_listeners(&mut self) -> &mut Self {
        self.listeners.clear();
        self
    }

    pub fn media_recorder(&self) -> &MediaRecorder {
        &self.media_recorder
    }

    pub fn media_recorder_mut(&mut self) -> &mut MediaRecorder {
        &mut self.media_recorder
    }

    pub fn recorded_chunks(&self) -> &Vec<u8> {
        &self.recorded_chunks
    }

    pub fn recorded_chunks_mut(&mut self) -> &mut Vec<u8> {
        &mut self.recorded_chunks
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    pub fn set_is_recording(&mut self, is_recording: bool) {
        self.is_recording = is_recording;
    }
}
