use std::ops::{Deref, DerefMut};

use crate::{JsProgramLink, ProgramLinkBuilder};
use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub type JsProgramLinkBuilderInner = ProgramLinkBuilder<String, String, String>;

#[wasm_bindgen]
pub struct JsProgramLinkBuilder(JsProgramLinkBuilderInner);

#[wasm_bindgen]
impl JsProgramLinkBuilder {
    pub fn default() -> Self {
        Self(JsProgramLinkBuilderInner::default())
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_program_id(mut self, program_id: String) -> Self {
        self.deref_mut().set_program_id(program_id);
        self
    }

    pub fn set_vertex_shader_id(mut self, vertex_shader_id: String) -> Self {
        self.deref_mut().set_vertex_shader_id(vertex_shader_id);
        self
    }

    pub fn set_fragment_shader_id(mut self, fragment_shader_id: String) -> Self {
        self.deref_mut().set_fragment_shader_id(fragment_shader_id);
        self
    }

    pub fn set_transform_feedback_varyings(mut self, transform_feedback_varyings: Array) -> Self {
        let string_vec: Vec<String> = js_sys::try_iter(transform_feedback_varyings.as_ref())
            .unwrap()
            .expect("set_transform_feedback_varyings should be passed an array of strings")
            .into_iter()
            .map(|el| {
                JsValue::as_string(&el
                    .expect("Each element in the array passed to set_transform_feedback_varyings should be a string")).unwrap()
            })
            .collect();

        self.deref_mut().set_transform_feedback_varyings(string_vec);

        self
    }

    pub fn build(self) -> Result<JsProgramLink, String> {
        self.0
            .build()
            .map(Into::into)
            .map_err(|err| err.to_string())
    }
}

impl Deref for JsProgramLinkBuilder {
    type Target = JsProgramLinkBuilderInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsProgramLinkBuilder {
    fn deref_mut(&mut self) -> &mut JsProgramLinkBuilderInner {
        &mut self.0
    }
}
