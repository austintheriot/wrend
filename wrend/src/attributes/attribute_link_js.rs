use std::ops::{Deref, DerefMut};

use js_sys::{Array, Function};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{utils, AttributeLink, AttributeLocation};

pub type AttributeLinkJsInner = AttributeLink<String, String, String>;

#[wasm_bindgen(js_name = AttributeLink)]
pub struct AttributeLinkJs(AttributeLinkJsInner);

#[wasm_bindgen(js_class = AttributeLink)]
impl AttributeLinkJs {
    pub fn new(
        vao_ids: Array,
        buffer_id: String,
        attribute_id: String,
        attribute_create_callback: Function,
    ) -> Self {
        let vao_ids = utils::js_array_to_vec_strings(vao_ids);
        Self(AttributeLinkJsInner::new(
            vao_ids,
            buffer_id,
            attribute_id,
            attribute_create_callback,
        ))
    }

    pub fn vao_ids(&self) -> Array {
        let ids = self.deref().vao_ids();
        utils::strings_to_js_array(ids)
    }

    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().to_owned()
    }

    pub fn attribute_id(&self) -> String {
        self.deref().attribute_id().to_owned()
    }

    pub fn create_callback(&self) -> Option<Function> {
        self.deref().create_callback().js_function()
    }

    pub fn create_attribute(
        &self,
        gl: WebGl2RenderingContext,
        now: f64,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
    ) {
        self.deref()
            .create_attribute(gl, now, webgl_buffer, attribute_location)
    }
}

impl AttributeLinkJs {
    pub fn inner(self) -> AttributeLinkJsInner {
        self.0
    }
}

impl Deref for AttributeLinkJs {
    type Target = AttributeLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributeLinkJs {
    fn deref_mut(&mut self) -> &mut AttributeLinkJsInner {
        &mut self.0
    }
}
