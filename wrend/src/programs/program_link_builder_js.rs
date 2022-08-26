use crate::{ProgramLinkJs, ProgramLinkBuilder};
use js_sys::Array;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub type ProgramLinkBuilderJsInner = ProgramLinkBuilder<String, String, String>;

#[wasm_bindgen(js_name = ProgramLinkBuilder)]
pub struct ProgramLinkJsBuilder(ProgramLinkBuilderJsInner);

#[wasm_bindgen(js_class = ProgramLinkBuilder)]
impl ProgramLinkJsBuilder {
    pub fn default() -> Self {
        Self(ProgramLinkBuilderJsInner::default())
    }

    #[wasm_bindgen(constructor)]
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

    pub fn build(self) -> Result<ProgramLinkJs, String> {
        self.0
            .build()
            .map(Into::into)
            .map_err(|err| err.to_string())
    }
}

impl Deref for ProgramLinkJsBuilder {
    type Target = ProgramLinkBuilderJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ProgramLinkJsBuilder {
    fn deref_mut(&mut self) -> &mut ProgramLinkBuilderJsInner {
        &mut self.0
    }
}

impl From<ProgramLinkBuilder<String, String, String>> for ProgramLinkJsBuilder {
    fn from(program_link: ProgramLinkBuilder<String, String, String>) -> Self {
        Self(program_link)
    }
}
