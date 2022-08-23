use crate::Attribute;
use wasm_bindgen::prelude::wasm_bindgen;

pub type JsAttributeInner = Attribute<String, String, String>;

#[wasm_bindgen]
pub struct JsAttribute(JsAttributeInner);

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
