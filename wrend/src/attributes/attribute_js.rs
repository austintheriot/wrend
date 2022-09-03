use std::ops::{Deref, DerefMut};

use crate::{Attribute, AttributeLocation, StringArray};
use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::WebGlBuffer;

pub type AttributeJsInner = Attribute<String, String, String>;

#[wasm_bindgen(inspectable, js_name = Attribute)]
pub struct AttributeJs(AttributeJsInner);

#[wasm_bindgen(js_class = Attribute)]
impl AttributeJs {
    #[wasm_bindgen(js_name = VAOIds)]
    pub fn vao_ids(&self) -> StringArray {
        let vao_ids: Vec<JsValue> = self
            .deref()
            .vao_ids()
            .iter()
            .map(|s| JsValue::from_str(s))
            .collect();

        Array::from_iter(vao_ids)
            .dyn_into()
            .expect("Should be able to convert Array of VAOIds into StringArray")
    }

    #[wasm_bindgen(js_name = bufferId)]
    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().clone()
    }

    #[wasm_bindgen(js_name = webglBuffer)]
    pub fn webgl_buffer(&self) -> WebGlBuffer {
        self.deref().webgl_buffer().clone()
    }

    #[wasm_bindgen(js_name = attributeLocation)]
    pub fn attribute_location(&self) -> AttributeLocation {
        *self.deref().attribute_location()
    }
}

impl Deref for AttributeJs {
    type Target = AttributeJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributeJs {
    fn deref_mut(&mut self) -> &mut AttributeJsInner {
        &mut self.0
    }
}

impl From<AttributeJsInner> for AttributeJs {
    fn from(js_attribute_inner: AttributeJsInner) -> Self {
        Self(js_attribute_inner)
    }
}

impl From<&AttributeJsInner> for AttributeJs {
    fn from(js_attribute_inner: &AttributeJsInner) -> Self {
        Self(js_attribute_inner.to_owned())
    }
}
