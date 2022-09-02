mod animation;
mod attributes;
mod buffers;
mod callbacks;
mod constants;
mod framebuffers;
mod ids;
mod math;
mod programs;
mod recording;
mod renderer_data;
mod renderers;
mod shaders;
mod textures;
mod transform_feedback;
mod uniforms;
mod utils;

pub use animation::*;
pub use attributes::*;
pub use buffers::*;
pub use callbacks::*;
pub use constants::*;
pub use framebuffers::*;
pub use ids::*;
pub use math::*;
pub use programs::*;
pub use recording::*;
pub use renderer_data::*;
pub use renderers::*;
pub use shaders::*;
pub use textures::*;
pub use transform_feedback::*;
pub use uniforms::*;
pub use utils::*;

use wasm_bindgen::prelude::wasm_bindgen;

// Called once--when the wasm module is instantiated.
// Allows logging helpful errors as well as providing standard logging output
#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
}
