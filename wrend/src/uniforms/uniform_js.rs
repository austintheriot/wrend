use crate::{utils, Uniform};
use js_sys::{Array, Map};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;

pub type UniformJsInner = Uniform<String, String>;

#[wasm_bindgen(js_name = Uniform)]
pub struct UniformJs(UniformJsInner);

#[wasm_bindgen(js_class = Uniform)]
impl UniformJs {
    #[wasm_bindgen(js_name = programIds)]
    pub fn program_ids(&self) -> Array {
        utils::strings_to_js_array(self.deref().program_ids())
    }

    #[wasm_bindgen(js_name = uniformId)]
    pub fn uniform_id(&self) -> String {
        self.deref().uniform_id().to_owned()
    }

    #[wasm_bindgen(js_name = uniformLocations)]
    pub fn uniform_locations(&self) -> Map {
        utils::hash_map_to_js_map(self.deref().uniform_locations())
    }
}

impl UniformJs {
    pub fn inner(self) -> UniformJsInner {
        self.0
    }
}

impl From<UniformJsInner> for UniformJs {
    fn from(js_uniform_inner: UniformJsInner) -> Self {
        Self(js_uniform_inner)
    }
}

impl From<&UniformJsInner> for UniformJs {
    fn from(js_uniform_inner: &UniformJsInner) -> Self {
        Self(js_uniform_inner.to_owned())
    }
}

impl Deref for UniformJs {
    type Target = UniformJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UniformJs {
    fn deref_mut(&mut self) -> &mut UniformJsInner {
        &mut self.0
    }
}
