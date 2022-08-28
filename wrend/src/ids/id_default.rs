use wasm_bindgen::prelude::wasm_bindgen;

use super::{id::Id, id_name::IdName};

/// Default ID that can be used when no id has been specified by the consuming application
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct IdDefault;

impl Id for IdDefault {}

impl IdName for IdDefault {
    fn name(&self) -> String {
        String::from("id_default")
    }
}
