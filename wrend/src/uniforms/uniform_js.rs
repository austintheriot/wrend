use crate::{utils, StringArray, Uniform, UniformLocationsMap};

use std::ops::{Deref, DerefMut};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};

pub type UniformJsInner = Uniform<String, String>;

#[wasm_bindgen(js_name = Uniform)]
pub struct UniformJs(UniformJsInner);

#[wasm_bindgen(js_class = Uniform)]
impl UniformJs {
    #[wasm_bindgen(js_name = programIds)]
    pub fn program_ids(&self) -> StringArray {
        utils::strings_to_js_array(self.deref().program_ids())
    }

    #[wasm_bindgen(js_name = uniformId)]
    pub fn uniform_id(&self) -> String {
        self.deref().uniform_id().to_owned()
    }

    #[wasm_bindgen(js_name = uniformLocations)]
    pub fn uniform_locations(&self) -> UniformLocationsMap {
        utils::hash_map_to_js_map(self.deref().uniform_locations())
            .dyn_into()
            .expect("Should be able to convert Map into UniformLocationsMap")
    }
}

impl UniformJs {
    pub fn into_inner(self) -> UniformJsInner {
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
