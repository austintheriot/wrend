use crate::Uniform;
use js_sys::{Array, Map, Object};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
pub type JsUniformInner = Uniform<String, String, Object>;

#[wasm_bindgen(js_name = Uniform)]
pub struct JsUniform(JsUniformInner);

#[wasm_bindgen(js_class = Uniform)]
impl JsUniform {
    pub fn program_ids(&self) -> Array {
        let program_ids: Vec<JsValue> = self
            .deref()
            .program_ids()
            .iter()
            .map(|s| JsValue::from_str(s))
            .collect();
        let mut array = Array::new();
        array.extend(program_ids);
        array
    }

    pub fn uniform_id(&self) -> String {
        self.deref().uniform_id().to_owned()
    }

    pub fn uniform_locations(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().uniform_locations().iter() {
            map.set(&JsValue::from_str(key), value.as_ref());
        }

        map
    }
}

impl From<JsUniformInner> for JsUniform {
    fn from(js_uniform_inner: JsUniformInner) -> Self {
        Self(js_uniform_inner)
    }
}

impl From<&JsUniformInner> for JsUniform {
    fn from(js_uniform_inner: &JsUniformInner) -> Self {
        Self(js_uniform_inner.to_owned())
    }
}

impl Deref for JsUniform {
    type Target = JsUniformInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsUniform {
    fn deref_mut(&mut self) -> &mut JsUniformInner {
        &mut self.0
    }
}
