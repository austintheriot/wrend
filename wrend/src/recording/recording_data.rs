use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlCanvasElement, MediaRecorder, MediaRecorderOptions, MediaStream};

#[wasm_bindgen(module = "/src/recording/captureStream.js")]
extern "C" {
    fn captureStreamFromCanvas(canvas: HtmlCanvasElement) -> MediaStream;
}

#[derive(Clone)]
pub struct RecordingData {
    recorded_chunks: Vec<u8>,
    media_stream: MediaStream,
    media_recorder: MediaRecorder,
}

impl RecordingData {
    /// Creates a `MediaStream` and `MediaRecorder` that is ready to being recording video
    /// from the canvas.
    pub fn new(canvas: impl AsRef<HtmlCanvasElement>) -> Self {
        let canvas = canvas.as_ref();
        let media_stream = captureStreamFromCanvas(canvas.clone());
        let mut media_recorder_options = MediaRecorderOptions::new();
        media_recorder_options.mime_type("video/webm; codecs=vp9");

        let media_recorder = MediaRecorder::new_with_media_stream_and_media_recorder_options(
            &media_stream,
            &media_recorder_options,
        )
        .expect("Should be able to build media recorder");

        Self {
            media_stream,
            media_recorder,
            recorded_chunks: Vec::new(),
        }
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
}
