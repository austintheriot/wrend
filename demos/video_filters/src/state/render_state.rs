use crate::graphics::FilterType;
use web_sys::HtmlVideoElement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderState {
    filter_type: FilterType,
    src_video: HtmlVideoElement,
}

impl RenderState {
    pub fn new(src_video: HtmlVideoElement) -> Self {
        Self {
            filter_type: FilterType::default(),
            src_video,
        }
    }

    pub fn src_video(&self) -> &HtmlVideoElement {
        &self.src_video
    }
}
