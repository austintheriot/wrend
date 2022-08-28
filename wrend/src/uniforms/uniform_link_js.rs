use std::ops::{Deref, DerefMut};

use crate::{utils, UniformLink};
use js_sys::{Array, Function, Object};
use wasm_bindgen::prelude::wasm_bindgen;

pub type UniformLinkJsInner = UniformLink<String, String, Object>;

#[wasm_bindgen(js_name = UniformLink)]
pub struct UniformLinkJs(UniformLinkJsInner);

#[wasm_bindgen(js_class = UniformLink)]
impl UniformLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(program_ids: Array, uniform_id: String, initialize_callback: Function) -> Self {
        let program_ids = utils::js_array_to_vec_strings(program_ids);
        Self(UniformLinkJsInner::new(
            program_ids,
            uniform_id,
            initialize_callback,
        ))
    }

    pub fn program_ids(&self) -> Array {
        utils::strings_to_js_array(self.deref().program_ids())
    }

    pub fn uniform_id(&self) -> String {
        self.deref().uniform_id().to_owned()
    }

    pub fn initialize_callback(&self) -> Option<Function> {
        self.deref().initialize_callback().b().map(Function::from)
    }

    pub fn set_initialize_callback(mut self, callback: Function) -> Self {
        self.deref_mut().set_initialize_callback(callback);
        self
    }

    pub fn should_update_callback(&self) -> Option<Function> {
        self.deref()
            .should_update_callback()
            .and_then(|callback| callback.js_function())
    }

    pub fn set_should_update_callback(mut self, callback: Function) -> Self {
        self.deref_mut().set_should_update_callback(callback);
        self
    }

    pub fn set_update_callback(mut self, callback: Function) -> Self {
        self.deref_mut().set_update_callback(callback);
        self
    }

    pub fn update_callback(&self) -> Option<Function> {
        self.deref()
            .update_callback()
            .and_then(|callback| callback.js_function())
    }

    pub fn use_init_callback_for_update(&self) -> bool {
        self.deref().use_init_callback_for_update()
    }

    pub fn set_use_init_callback_for_update(mut self, use_init_callback_for_update: bool) -> Self {
        self.deref_mut()
            .set_use_init_callback_for_update(use_init_callback_for_update);
        self
    }
}

impl From<UniformLinkJs> for UniformLinkJsInner {
    fn from(uniform_link_js: UniformLinkJs) -> Self {
        uniform_link_js.inner()
    }
}

impl UniformLinkJs {
    pub fn inner(self) -> UniformLinkJsInner {
        self.0
    }
}

impl From<UniformLinkJsInner> for UniformLinkJs {
    fn from(js_program_link_inner: UniformLinkJsInner) -> Self {
        Self(js_program_link_inner)
    }
}

impl Deref for UniformLinkJs {
    type Target = UniformLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UniformLinkJs {
    fn deref_mut(&mut self) -> &mut UniformLinkJsInner {
        &mut self.0
    }
}
