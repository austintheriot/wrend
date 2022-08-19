use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{BlobEvent, HtmlCanvasElement, MediaRecorder, MediaRecorderOptions, MediaStream};

use crate::{Listener, RecordingUrl};

#[wasm_bindgen(module = "/src/recording/captureStream.js")]
extern "C" {
    fn captureStreamFromCanvas(canvas: HtmlCanvasElement) -> MediaStream;
}

pub struct RecordingData {
    recorded_chunks: Vec<u8>,
    media_stream: MediaStream,
    media_recorder: MediaRecorder,
    recording_data_available_listener: Option<Listener<MediaRecorder, BlobEvent>>,
    recording_stop_listener: Option<Listener<MediaRecorder, BlobEvent>>,
    recording_url: Option<RecordingUrl>,
}

impl RecordingData {
    pub const VIDEO_TYPE: &'static str = "video/webm";

    /// Creates a `MediaStream` and `MediaRecorder` that is ready to being recording video
    /// from the canvas.
    pub fn new(canvas: impl AsRef<HtmlCanvasElement>) -> Self {
        let canvas = canvas.as_ref();
        let media_stream = captureStreamFromCanvas(canvas.clone());
        let mut media_recorder_options = MediaRecorderOptions::new();
        let mime_type = format!("{}; codecs=vp9", Self::VIDEO_TYPE);
        media_recorder_options.mime_type(&mime_type);

        let media_recorder = MediaRecorder::new_with_media_stream_and_media_recorder_options(
            &media_stream,
            &media_recorder_options,
        )
        .expect("Should be able to build media recorder");

        Self {
            media_stream,
            media_recorder,
            recorded_chunks: Vec::new(),
            recording_data_available_listener: None,
            recording_stop_listener: None,
            recording_url: None,
        }
    }

    pub fn set_recording_data_available_listener(
        &mut self,
        recording_data_available_listener: Option<Listener<MediaRecorder, BlobEvent>>,
    ) {
        self.recording_data_available_listener = recording_data_available_listener;
    }

    pub fn set_recording_stop_listener(
        &mut self,
        recording_stop_listener: Option<Listener<MediaRecorder, BlobEvent>>,
    ) {
        self.recording_stop_listener = recording_stop_listener;
    }

    pub fn media_recorder(&self) -> &MediaRecorder {
        &self.media_recorder
    }

    pub fn media_recorder_mut(&mut self) -> &mut MediaRecorder {
        &mut self.media_recorder
    }

    pub fn media_stream(&self) -> &MediaStream {
        &self.media_stream
    }

    pub fn media_stream_mut(&mut self) -> &mut MediaStream {
        &mut self.media_stream
    }

    pub fn recorded_chunks(&self) -> &Vec<u8> {
        &self.recorded_chunks
    }

    pub fn recorded_chunks_mut(&mut self) -> &mut Vec<u8> {
        &mut self.recorded_chunks
    }

    pub fn recording_url(&self) -> &Option<RecordingUrl> {
        &self.recording_url
    }

    pub fn set_recording_url(&mut self, recording_url: impl Into<RecordingUrl>) {
        self.recording_url = Some(recording_url.into());
    }
}
