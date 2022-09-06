#![warn(missing_docs)]

//! Wrend is a ***W***ebGL2 ***Rend***ering library for making Rust/JavaScript + WebGL development easier and safer.
//! 
//! Wrend provides an organized system for building custom rendering pipelines: you tell Wrend *how* all of your graphical resources
//! relate to one another through user-specified ids and callbacks, and it does all the work of actually putting things together.
//! It also comes with some out-of-the-box nice-to-have abstractions like recording canvas output and animation frame scheduling,
//! as well as automatic clean-up.
//! 
//! Though most of the demo app examples are built using Yew, `wrend` itself is framework agnostic  and is designed 
//! to be used in a variety of settings with diverse rendering pipelines, including contexts like React and 
//! raw HTML & JavaScript.
//! 
//! ## Overview
//! 
//! ### Links
//! 
//! The fundamental organizing components of Wrend are `links`, such as [`ProgramLink`], [`AttributeLink`], 
//! and [`UniformLink`],  which get appended to a [RendererDataBuilder]. These links tell `wrend` how your data 
//! should be created and how each resource relates to all the other resources in your pipeline. 
//! 
//! ### Callbacks
//! 
//! Many links accept some sort of callback, which is used to create a particular resource in your  build pipeline.
//! 
//! For example, [`BufferLink`] accepts a [`BufferCreateCallback`], which is called during the build process to 
//! acquire a [`web_sys::WebGlBuffer`]. In this callback, you are free to initialize your [`web_sys::WebGlBuffer`]
//! however you like.
//! 
//! ### Ids
//! 
//! Most resources such as shaders, [`Uniform`]s and [`Attribute`]s retrieve resources using unique [`Id`]s, which can be 
//! any data type that implements the [`Id`] trait. These Ids help Wrend understand how your data fits together. 
//! 
//! For example, you can load shaders into the build pipeline using [`RendererDataBuilder::add_vertex_shader_src`].
//! Then, when creating a [`ProgramLink`], you can refer to that shader using its `VertexShaderId` to link that shader 
//! to any number of programs you create.
//! 
//! ### Build
//! 
//! Once all resources and `links` have been added to the [RendererDataBuilder], the pipeline can be built
//! on using [RendererDataBuilder::build_renderer].
//! 
//! # Panics
//! 
//! There are very few locations in which Rust code can panic in `wrend`, and those that exist are being slimmed down. 
//! 
//! The primary locations that *can* are where JavaScript types such as arrays are passed into Rust--because these types must be converted
//! to WebAssembly, there is currently the chance for panic if the wrong type is supplied. If using TypeScript, you should see TypeScript errors
//! for any incorrect types supplied, as the library as a whole is strongly typed.
//! 
//! A long term goal of `wrend` is to provide matchable errors (or `catch`able errors in JavaScript) for all fallible operations.
//! 
//! # Example
//! 
//! todo
//! 
//! # Demos
//! 
//! todo
//! 
//! ## Future Work
//! 
//! Currently, wrend only supports build pipelines where all resources are initialized up front. 
//! That is, no *new* textures, buffers, uniforms can be added after the pipeline has been initialized.

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
mod types;
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
pub use types::*;
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
