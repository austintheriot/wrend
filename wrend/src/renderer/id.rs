use std::fmt::Debug;
use std::hash::Hash;

/// Collection of traits that make something useful as an id for programs, shaders, uniforms, etc.
pub trait Id: 'static + Hash + PartialEq + Eq + Clone + Debug + Default {}
