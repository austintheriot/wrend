use std::ops::{Deref, DerefMut};

use crate::{JsProgramLinkBuilder, ProgramLink};
use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub type JsProgramLinkInner = ProgramLink<String, String, String>;

#[wasm_bindgen]
pub struct JsProgramLink(JsProgramLinkInner);

#[wasm_bindgen]
impl JsProgramLink {
    pub fn new(program_id: String, vertex_shader_id: String, fragment_shader_id: String) -> Self {
        Self(JsProgramLinkInner::new(
            program_id,
            vertex_shader_id,
            fragment_shader_id,
        ))
    }

    pub fn program_id(&self) -> String {
        self.deref().program_id().to_string()
    }

    pub fn vertex_shader_id(&self) -> String {
        self.deref().vertex_shader_id().to_string()
    }

    pub fn fragment_shader_id(&self) -> String {
        self.deref().fragment_shader_id().to_string()
    }

    pub fn transform_feedback_varyings(&self) -> Array {
        let string_vec: Vec<JsValue> = self
            .deref()
            .transform_feedback_varyings()
            .to_owned()
            .into_iter()
            .map(JsValue::from)
            .collect();
        let mut array = Array::new();
        array.extend(string_vec);
        array
    }

    pub fn builder() -> JsProgramLinkBuilder {
        JsProgramLinkBuilder::default()
    }
}

impl From<JsProgramLinkInner> for JsProgramLink {
    fn from(js_program_link_inner: JsProgramLinkInner) -> Self {
        Self(js_program_link_inner)
    }
}

impl Deref for JsProgramLink {
    type Target = JsProgramLinkInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsProgramLink {
    fn deref_mut(&mut self) -> &mut JsProgramLinkInner {
        &mut self.0
    }
}
