use std::ops::{Deref, DerefMut};

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{utils, AttributeCreateCallbackJs, AttributeLink, AttributeLocation, StringArray};

pub type AttributeLinkJsInner = AttributeLink<String, String, String>;

#[wasm_bindgen(js_name = AttributeLink)]
pub struct AttributeLinkJs(AttributeLinkJsInner);

#[wasm_bindgen(js_class = AttributeLink)]
impl AttributeLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        vao_ids: StringArray,
        buffer_id: String,
        attribute_id: String,
        attribute_create_callback: AttributeCreateCallbackJs,
    ) -> Self {
        let vao_ids = utils::js_array_to_vec_strings(&vao_ids);
        Self(AttributeLinkJsInner::new(
            vao_ids,
            buffer_id,
            attribute_id,
            attribute_create_callback,
        ))
    }

    #[wasm_bindgen(js_name = VAOIds)]
    pub fn vao_ids(&self) -> StringArray {
        let ids = self.deref().vao_ids();
        utils::strings_to_js_array(ids)
    }

    #[wasm_bindgen(js_name = bufferId)]
    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().to_owned()
    }

    #[wasm_bindgen(js_name = attributeId)]
    pub fn attribute_id(&self) -> String {
        self.deref().attribute_id().to_owned()
    }

    #[wasm_bindgen(js_name = createCallback)]
    pub fn create_callback(&self) -> Option<AttributeCreateCallbackJs> {
        self.deref().create_callback().js_inner_owned()
    }

    #[wasm_bindgen(js_name = createAttribute)]
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
    pub fn into_inner(self) -> AttributeLinkJsInner {
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
