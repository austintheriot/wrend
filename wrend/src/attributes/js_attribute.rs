use std::ops::{Deref, DerefMut};

use crate::{Attribute, AttributeLocation};
use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::WebGlBuffer;

pub type JsAttributeInner = Attribute<String, String, String>;

#[wasm_bindgen(js_name = Attribute)]
pub struct JsAttribute(JsAttributeInner);

#[wasm_bindgen(js_class = Attribute)] 
impl JsAttribute {
    pub fn vao_ids(&self) -> Array {
        let vao_ids: Vec<JsValue> = self.deref().vao_ids().iter().map(|s| JsValue::from_str(s)).collect();
        let array = Array::from_iter(vao_ids);
        array
    }

    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().clone()
    }

    pub fn webgl_buffer(&self) -> WebGlBuffer {
        self.deref().webgl_buffer().clone()
    }

    pub fn attribute_location(&self) -> AttributeLocation {
        *self.deref().attribute_location()
    }
}

impl Deref for JsAttribute {
    type Target = JsAttributeInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsAttribute {
    fn deref_mut(&mut self) -> &mut JsAttributeInner {
        &mut self.0
    }
}

impl From<JsAttributeInner> for JsAttribute {
    fn from(js_attribute_inner: JsAttributeInner) -> Self {
        Self(js_attribute_inner)
    }
}

impl From<&JsAttributeInner> for JsAttribute {
    fn from(js_attribute_inner: &JsAttributeInner) -> Self {
        Self(js_attribute_inner.to_owned())
    }
}
