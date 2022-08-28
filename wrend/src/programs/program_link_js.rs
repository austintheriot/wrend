use std::ops::{Deref, DerefMut};

use crate::{ProgramLink, ProgramLinkJsBuilder};
use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub type ProgramLinkJsInner = ProgramLink<String, String, String>;

#[wasm_bindgen(js_name = ProgramLink)]
pub struct ProgramLinkJs(ProgramLinkJsInner);

#[wasm_bindgen(js_class = ProgramLink)]
impl ProgramLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(program_id: String, vertex_shader_id: String, fragment_shader_id: String) -> Self {
        Self(ProgramLinkJsInner::new(
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
            .iter()
            .map(|s| JsValue::from_str(s))
            .collect();

        Array::from_iter(string_vec)
    }

    pub fn builder() -> ProgramLinkJsBuilder {
        ProgramLinkJsBuilder::default()
    }
}

impl ProgramLinkJs {
    pub fn inner(self) -> ProgramLinkJsInner {
        self.0
    }
}

impl From<ProgramLinkJsInner> for ProgramLinkJs {
    fn from(js_program_link_inner: ProgramLinkJsInner) -> Self {
        Self(js_program_link_inner)
    }
}

impl Deref for ProgramLinkJs {
    type Target = ProgramLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ProgramLinkJs {
    fn deref_mut(&mut self) -> &mut ProgramLinkJsInner {
        &mut self.0
    }
}
