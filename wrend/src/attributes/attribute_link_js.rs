use std::ops::{Deref, DerefMut};

use js_sys::{Array, Object};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::AttributeLink;

pub type AttributeLinkJsInner = AttributeLink<String, String, String, Object>;

#[wasm_bindgen(js_name = AttributeLink)]
pub struct AttributeLinkJs(AttributeLinkJsInner);

#[wasm_bindgen(js_class = AttributeLink)]
impl AttributeLinkJs {
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
