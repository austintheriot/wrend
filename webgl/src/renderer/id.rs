use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Collection of traits that make something usable as an id for programs, shaders, uniforms, etc.
pub trait Id: Hash + PartialEq + Eq + Clone + Debug + Default + Display {}
