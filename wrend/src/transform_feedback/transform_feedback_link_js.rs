use std::ops::{Deref, DerefMut};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::TransformFeedbackLink;

pub type TransformFeedbackLinkJsInner = TransformFeedbackLink<String>;

#[wasm_bindgen(js_name = TransformFeedbackLink)]
pub struct TransformFeedbackLinkJs(TransformFeedbackLinkJsInner);

#[wasm_bindgen(js_class = TransformFeedbackLink)]
impl TransformFeedbackLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(transform_feedback_id: String) -> Self {
        Self(TransformFeedbackLinkJsInner::new(transform_feedback_id))
    }

    #[wasm_bindgen(js_name = transformFeedbackId)]
    pub fn transform_feedback_id(&self) -> String {
        self.deref().transform_feedback_id().to_owned()
    }
}

impl TransformFeedbackLinkJs {
    pub fn inner(self) -> TransformFeedbackLinkJsInner {
        self.0
    }
}

impl Deref for TransformFeedbackLinkJs {
    type Target = TransformFeedbackLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TransformFeedbackLinkJs {
    fn deref_mut(&mut self) -> &mut TransformFeedbackLinkJsInner {
        &mut self.0
    }
}

impl From<TransformFeedbackLinkJs> for TransformFeedbackLinkJsInner {
    fn from(buffer_link_js: TransformFeedbackLinkJs) -> Self {
        buffer_link_js.inner()
    }
}
