use std::ops::{Deref, DerefMut};

use js_sys::{Array, Object};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::AttributeLink;

pub type JsAttributeLinkInner = AttributeLink<String, String, String, Object>;

#[wasm_bindgen(js_name = AttributeLink)]
pub struct JsAttributeLink(JsAttributeLinkInner);

#[wasm_bindgen(js_class = AttributeLink)]
impl JsAttributeLink {
    pub fn vao_ids(&self) -> Array {
        let string_vec: Vec<JsValue> = self
            .deref()
            .vao_ids()
            .iter()
            .map(|el| JsValue::from_str(el))
            .collect();

        let array = Array::from_iter(string_vec);

        array
    }

    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().to_owned()
    }

    pub fn attribute_id(&self) -> String {
        self.deref().attribute_id().to_owned()
    }
}

impl JsAttributeLink {
    pub fn inner(self) -> JsAttributeLinkInner {
        self.0
    }
}

impl Deref for JsAttributeLink {
    type Target = JsAttributeLinkInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsAttributeLink {
    fn deref_mut(&mut self) -> &mut JsAttributeLinkInner {
        &mut self.0
    }
}
